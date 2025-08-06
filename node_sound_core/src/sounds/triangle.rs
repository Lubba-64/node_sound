use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::SetSpeed};
use rodio::Source;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct TriangleWave {
    freq: f32,
    num_sample: usize,
    speed: f32,
    uses_speed: bool,
}

impl TriangleWave {
    #[inline]
    pub fn new(freq: f32, uses_speed: bool) -> Self {
        Self {
            freq,
            num_sample: 0,
            speed: 1.0,
            uses_speed,
        }
    }
}

impl Iterator for TriangleWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        let value =
            (self.freq / self.speed / 2.0 * self.num_sample as f32) / DEFAULT_SAMPLE_RATE as f32;
        Some((((value % 1.0) - 0.5).abs() * 4.0) - 1.0)
    }
}

impl Source for TriangleWave {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
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

impl SetSpeed for TriangleWave {
    fn set_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
    }
}
