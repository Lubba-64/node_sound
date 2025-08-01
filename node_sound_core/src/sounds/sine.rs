use crate::constants::DEFAULT_SAMPLE_RATE;
use crate::sound_map::SetSpeed;
use rodio::Source;
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct SineWave {
    freq: f32,
    num_sample: usize,
    speed: f32,
    uses_speed: bool,
}

impl SineWave {
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

impl Iterator for SineWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        let value =
            2.0 * PI * self.freq / self.speed * self.num_sample as f32 / DEFAULT_SAMPLE_RATE as f32;
        Some(value.sin())
    }
}

impl Source for SineWave {
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

impl SetSpeed<f32> for SineWave {
    fn set_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
    }
}
