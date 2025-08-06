use rodio::Source;

use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::SetSpeed};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct SawToothWave {
    freq: f32,
    num_sample: usize,
    speed: f32,
    uses_speed: bool,
}

impl SawToothWave {
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

impl Iterator for SawToothWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = (self.freq * self.num_sample as f32) / self.speed / DEFAULT_SAMPLE_RATE as f32;
        Some((value % 2.0) - 1.0)
    }
}

impl Source for SawToothWave {
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

impl SetSpeed for SawToothWave {
    fn set_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
    }
}
