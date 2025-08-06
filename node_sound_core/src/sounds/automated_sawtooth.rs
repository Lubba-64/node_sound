use rodio::{Source, source::UniformSourceIterator};

use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::SetSpeed};
use std::time::Duration;

#[derive(Clone)]
pub struct AutomatedSawToothWave<T: rodio::Source<Item = f32>> {
    freq: UniformSourceIterator<T>,
    num_sample: usize,
    uses_speed: bool,
    speed: f32,
}

impl<T: rodio::Source<Item = f32>> AutomatedSawToothWave<T> {
    #[inline]
    pub fn new(freq: T, uses_speed: bool) -> Self {
        Self {
            freq: UniformSourceIterator::new(freq, 1, DEFAULT_SAMPLE_RATE),
            num_sample: 0,
            speed: 1.0,
            uses_speed,
        }
    }
}

impl<T: rodio::Source<Item = f32>> Iterator for AutomatedSawToothWave<T> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        self.freq.next().map(|freq| {
            let value = (freq * (self.num_sample as f32 % DEFAULT_SAMPLE_RATE as f32))
                / DEFAULT_SAMPLE_RATE as f32;
            (value % 2.0) - 1.0
        })
    }
}

impl<T: rodio::Source<Item = f32>> Source for AutomatedSawToothWave<T> {
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

impl<I: Source<Item = f32>> SetSpeed for AutomatedSawToothWave<I> {
    fn set_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
    }
}
