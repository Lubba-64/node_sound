use rodio::Source;
use rodio::source::UniformSourceIterator;
use rustfft::{FftPlanner, num_complex::Complex};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct FftPitchShifter<I>
where
    I: Source<Item = f32>,
{
    inner: UniformSourceIterator<I, f32>,
    output_buffer: VecDeque<f32>,
    input_buffer: Vec<f32>,
    fft: Arc<dyn rustfft::Fft<f32>>,
    ifft: Arc<dyn rustfft::Fft<f32>>,
    window: Vec<f32>,
    fft_size: usize,
    hop_size: usize,
    target_frequency: Option<f32>,
    last_phase: Vec<f32>,
    accum_phase: Vec<f32>,
    target_sample_rate: u32,
    samples_since_last_process: usize,
    current_pitch_ratio: f32,
    detected_frequency: Option<f32>,
}

impl<I> FftPitchShifter<I>
where
    I: Source<Item = f32>,
{
    pub fn new(source: I, fft_size: usize, target_sample_rate: u32) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of two");

        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_size);
        let ifft = planner.plan_fft_inverse(fft_size);

        // Hann window
        let window: Vec<f32> = (0..fft_size)
            .map(|i| {
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (fft_size - 1) as f32).cos())
            })
            .collect();

        let hop_size = fft_size / 4; // 25% overlap

        Self {
            inner: UniformSourceIterator::new(source, 1, target_sample_rate),
            output_buffer: VecDeque::with_capacity(fft_size * 2),
            input_buffer: vec![0.0; fft_size],
            fft,
            ifft,
            window,
            fft_size,
            hop_size,
            target_frequency: None,
            last_phase: vec![0.0; fft_size],
            accum_phase: vec![0.0; fft_size],
            target_sample_rate,
            samples_since_last_process: 0,
            current_pitch_ratio: 1.0,
            detected_frequency: None,
        }
    }

    pub fn set_target_frequency(mut self, frequency_hz: f32) -> Self {
        self.target_frequency = Some(frequency_hz);
        self
    }

    fn detect_frequency(&mut self, spectrum: &[Complex<f32>]) -> Option<f32> {
        // Find peak bin
        let (peak_bin, _) = spectrum
            .iter()
            .take(self.fft_size / 2)
            .enumerate()
            .max_by(|(_, a), (_, b)| a.norm().partial_cmp(&b.norm()).unwrap())
            .unwrap();

        // Calculate actual frequency using phase difference
        let phase_diff = spectrum[peak_bin].arg() - self.last_phase[peak_bin];
        let bin_center_freq = 2.0 * std::f32::consts::PI * peak_bin as f32 / self.fft_size as f32;
        let true_freq = bin_center_freq
            + (phase_diff - bin_center_freq * self.hop_size as f32) / self.hop_size as f32;

        Some(true_freq * self.target_sample_rate as f32 / (2.0 * std::f32::consts::PI))
    }

    fn calculate_pitch_ratio(&mut self, detected_freq: f32) -> f32 {
        self.target_frequency
            .map(|target| target / detected_freq)
            .unwrap_or(1.0)
    }

    fn process_frame(&mut self) {
        // Apply window
        let mut frame: Vec<_> = self
            .input_buffer
            .iter()
            .zip(&self.window)
            .map(|(s, w)| Complex::new(s * w, 0.0))
            .collect();

        // FFT
        self.fft.process(&mut frame);

        // Detect frequency and calculate ratio
        if let Some(detected) = self.detect_frequency(&frame) {
            self.detected_frequency = Some(detected);
            self.current_pitch_ratio = self.calculate_pitch_ratio(detected);
        }

        // Phase vocoder processing
        let mut new_frame = vec![Complex::new(0.0, 0.0); self.fft_size];
        for i in 0..self.fft_size / 2 {
            let mag = frame[i].norm();
            let phase = frame[i].arg();

            // Phase difference calculation
            let phase_diff = phase - self.last_phase[i];
            self.last_phase[i] = phase;

            let bin_center_freq = 2.0 * std::f32::consts::PI * i as f32 / self.fft_size as f32;
            let freq = bin_center_freq
                + (phase_diff - bin_center_freq * self.hop_size as f32) / self.hop_size as f32;

            // Apply pitch shift
            let new_bin = (i as f32 * self.current_pitch_ratio).round() as usize;
            if new_bin < self.fft_size / 2 {
                self.accum_phase[new_bin] += freq * self.hop_size as f32;
                new_frame[new_bin] = Complex::from_polar(mag, self.accum_phase[new_bin]);
                new_frame[self.fft_size - new_bin - 1] =
                    Complex::from_polar(mag, -self.accum_phase[new_bin]);
            }
        }

        // IFFT
        self.ifft.process(&mut new_frame);

        // Add to output buffer with overlap-add
        for (i, sample) in new_frame.iter().enumerate().take(self.hop_size) {
            let idx = self.output_buffer.len().saturating_sub(self.hop_size) + i;
            let windowed_sample = sample.re * self.window[i] / (self.fft_size as f32);

            if idx < self.output_buffer.len() {
                self.output_buffer[idx] += windowed_sample;
            } else {
                self.output_buffer.push_back(windowed_sample);
            }
        }

        // Shift input buffer
        self.input_buffer.copy_within(self.hop_size.., 0);
        self.input_buffer[self.fft_size - self.hop_size..].fill(0.0);
    }
}

impl<I> Iterator for FftPitchShifter<I>
where
    I: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // Only process new frame when we need to
        if self.output_buffer.is_empty() {
            // Fill input buffer
            while self.samples_since_last_process < self.hop_size {
                if let Some(sample) = self.inner.next() {
                    let pos = (self.fft_size - self.hop_size) + self.samples_since_last_process;
                    self.input_buffer[pos] = sample;
                    self.samples_since_last_process += 1;
                } else {
                    break;
                }
            }

            if self.samples_since_last_process >= self.hop_size {
                self.process_frame();
                self.samples_since_last_process = 0;
            }
        }

        self.output_buffer.pop_front()
    }
}

impl<I> Source for FftPitchShifter<I>
where
    I: Source<Item = f32>,
{
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.inner.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.target_sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
