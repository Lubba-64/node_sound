use rodio::Source;

use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::SetSpeed};
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct AutomatedModRaw<I: Source<Item = f32>, I2: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
    mod_by: UniformSourceIterator<I2, I2::Item>,
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>> AutomatedModRaw<I, I2> {
    #[inline]
    pub fn new(source: I, mod_by: I2) -> Self {
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
            mod_by: UniformSourceIterator::new(mod_by, 2, DEFAULT_SAMPLE_RATE),
        }
    }
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>> Iterator for AutomatedModRaw<I, I2> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        match (self.source.next(), self.mod_by.next()) {
            (Some(x), Some(mod_by)) => Some(x % mod_by),
            _ => None,
        }
    }
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>> Source for AutomatedModRaw<I, I2> {
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

impl<I: Source<Item = f32>, I2: Source<Item = f32>> SetSpeed<f32> for AutomatedModRaw<I, I2> {
    fn set_speed(&mut self, _speed: f32) {}
}
