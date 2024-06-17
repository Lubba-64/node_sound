use itertools::izip;
use realfft::{
    num_complex::{Complex, ComplexFloat},
    RealFftPlanner,
};
use rodio::Source;
use std::{f32::consts::PI, ops::Div};

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

const H: f32 = 2.0 * 2.0 * 2.0 * 2.0 * 2.0 * 2.0 * 2.0 * 2.0 * 2.0 * 2.0 * 2.0; // 2^11

fn hanning(n: f32, m: f32) -> f32 {
    0.3f32 - 0.5f32 * f32::cos((2.0f32 * PI * n) / (m - 1.0f32))
}

fn stretch(sound: Vec<f32>, factor: f32, window_size: usize, h: f32) -> Vec<f32> {
    let int_h = h as usize;
    let mut phase: Vec<f32> = Vec::with_capacity(window_size);
    let hanning_window: Vec<f32> = (0..window_size)
        .into_iter()
        .map(|x| hanning(x as f32, window_size as f32))
        .collect();

    let mut fft = RealFftPlanner::<f32>::new();
    let r2c_fft = fft.plan_fft_forward(window_size);
    let c2r_fft = fft.plan_fft_inverse(window_size);

    let mut result: Vec<f32> =
        Vec::with_capacity((sound.len() as f32 / factor + window_size as f32) as usize);
    for i in (0..sound.len() - window_size + int_h)
        .into_iter()
        .filter(|x| x % (h * factor) as usize == 0)
    {
        let a1 = &sound[i..i + window_size];
        let a2 = &sound[i + int_h..i + window_size + int_h];
        let mut b1: Vec<_> = a1
            .iter()
            .zip(hanning_window.clone())
            .map(|(x, y)| x * y)
            .collect();
        let mut s1: Vec<Complex<f32>> = Vec::with_capacity(window_size);
        let _ = r2c_fft.process(&mut b1, &mut s1);
        let mut b2: Vec<_> = a2
            .iter()
            .zip(hanning_window.clone())
            .map(|(x, y)| x * y)
            .collect();
        let mut s2: Vec<_> = Vec::with_capacity(window_size);
        let _ = r2c_fft.process(&mut b2, &mut s2);
        phase = izip!(s1, s2.clone(), phase.clone())
            .map(|(s1, s2, phase)| (complex_angle(s2 / s1) + phase) % 2.0f32 * PI)
            .collect();
        let mut rephase: Vec<_> = s2
            .clone()
            .iter()
            .zip(phase.clone())
            .map(|(s2, phase)| {
                Complex {
                    re: s2.abs(),
                    im: 0.0,
                } * (Complex { re: 0.0, im: 1.0 } * phase)
            })
            .collect();
        let mut a2_rephased = Vec::with_capacity(window_size);
        let _ = c2r_fft.process(&mut rephase, &mut a2_rephased);
        let i2 = (i as f32 / factor).round() as usize;
        let _ = result[i2..i2 + window_size]
            .iter_mut()
            .zip(hanning_window.iter().zip(a2_rephased).map(|(h, a2)| h * a2))
            .map(|(x, y)| *x += y);
    }
    return result;
}
// c = arctanb/a.
// a + bj
fn complex_angle<T: Div<Output = f32>>(c: Complex<T>) -> f32 {
    f32::atan(c.im / c.re)
}

fn speedx(sound: Vec<f32>, factor: f32) -> Vec<f32> {
    let pow_factor = 10_f32.powf(factor);
    (0..sound.len())
        .into_iter()
        .map(|x| f32::from((x as f32 * pow_factor).round() / pow_factor))
        .collect()
}

fn pitch_shift(sound: Vec<f32>, n: f32, window_size: usize) -> Vec<f32> {
    let factor = 2_f32.powf(1.0 * n / 12.0);
    let stretched = stretch(sound, factor, window_size, H);
    return speedx(stretched[window_size..].to_vec(), factor);
}

#[derive(Clone)]
pub struct PitchShifter<I: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
    shift: f32,
    in_buffer: Vec<f32>,
    out_buffer: Vec<f32>,
    buffer_size: usize,
}

impl<I: Source<Item = f32>> PitchShifter<I> {
    #[inline]
    pub fn new(source: I, shift: f32, buffer_size: usize) -> Self {
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
            shift,
            buffer_size: buffer_size * DEFAULT_SAMPLE_RATE as usize,
            in_buffer: vec![],
            out_buffer: vec![],
        }
    }
}

impl<I: Source<Item = f32>> Iterator for PitchShifter<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        if self.out_buffer.len() > 0 {
            self.in_buffer.push(self.source.next().unwrap_or(0.0));
            return Some(self.out_buffer.pop().unwrap());
        } else {
            self.out_buffer = pitch_shift(self.in_buffer.clone(), self.shift, self.buffer_size / 3);
            return Some(self.out_buffer.pop().unwrap());
        }
    }
}

impl<I: Source<Item = f32>> Source for PitchShifter<I> {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        2
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        DEFAULT_SAMPLE_RATE
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
