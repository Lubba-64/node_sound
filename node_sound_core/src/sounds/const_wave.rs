use rodio::Source;

use crate::constants::DEFAULT_SAMPLE_RATE;
use std::time::Duration;

#[derive(Clone)]
pub struct ConstWave {
    amplitude: f32,
}

impl ConstWave {
    #[inline]
    pub fn new(amplitude: f32) -> Self {
        Self { amplitude }
    }
}

impl Iterator for ConstWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        Some(self.amplitude)
    }
}

impl Source for ConstWave {
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
