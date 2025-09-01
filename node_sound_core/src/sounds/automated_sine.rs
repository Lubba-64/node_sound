use std::f32::consts::PI;
use std::time::Duration;

use rodio::Source;
use rodio::source::UniformSourceIterator;

use crate::constants::DEFAULT_SAMPLE_RATE;
use crate::sound_map::SetSpeed;

#[derive(Clone)]
pub struct AutomatedSineWave<T: rodio::Source<Item = f32>> {
    freq: UniformSourceIterator<T, f32>,
    phase: f32,
    uses_speed: bool,
    speed: f32,
}

impl<T: rodio::Source<Item = f32>> AutomatedSineWave<T> {
    #[inline]
    pub fn new(freq: T, uses_speed: bool) -> AutomatedSineWave<T> {
        AutomatedSineWave {
            freq: UniformSourceIterator::new(freq, 1, DEFAULT_SAMPLE_RATE),
            phase: 0.0,
            uses_speed,
            speed: 1.0,
        }
    }
}

impl<T: rodio::Source<Item = f32>> Iterator for AutomatedSineWave<T> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.freq.next().map(|freq| {
            let phase_increment = 2.0 * PI * freq / DEFAULT_SAMPLE_RATE as f32 / self.speed;
            self.phase = (self.phase + phase_increment) % (2.0 * PI);
            self.phase.sin()
        })
    }
}

impl<T: rodio::Source<Item = f32>> Source for AutomatedSineWave<T> {
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

impl<I: Source<Item = f32>> SetSpeed<f32> for AutomatedSineWave<I> {
    fn set_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
    }
}
