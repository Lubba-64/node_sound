use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct AutomatedMod<I: Source<Item = f32>, I2: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
    mod_by: UniformSourceIterator<I2, I2::Item>,
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>> AutomatedMod<I, I2> {
    #[inline]
    pub fn new(source: I, mod_by: I2) -> Self {
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
            mod_by: UniformSourceIterator::new(mod_by, 2, DEFAULT_SAMPLE_RATE),
        }
    }
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>> Iterator for AutomatedMod<I, I2> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let _next = self.source.next().unwrap_or(0.0);
        Some(_next - (_next % self.mod_by.next().unwrap_or(0.0)))
    }
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>> Source for AutomatedMod<I, I2> {
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
