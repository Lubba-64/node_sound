use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use rodio::Source;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct TriangleWave {
    freq: f32,
    num_sample: usize,
}

impl TriangleWave {
    #[inline]
    pub fn new(freq: f32) -> Self {
        Self {
            freq: freq,
            num_sample: 0,
        }
    }
}

impl Iterator for TriangleWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        let value = (self.freq * self.num_sample as f32) / DEFAULT_SAMPLE_RATE as f32;
        Some(((value % 1.0) - 0.5).abs())
    }
}

impl Source for TriangleWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
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
