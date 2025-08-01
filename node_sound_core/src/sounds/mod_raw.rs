use rodio::Source;

use crate::constants::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct RawMod<I: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
    mod_by: f32,
}

impl<I: Source<Item = f32>> RawMod<I> {
    #[inline]
    pub fn new(source: I, mod_by: f32) -> Self {
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
            mod_by,
        }
    }
}

impl<I: Source<Item = f32>> Iterator for RawMod<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        match self.source.next() {
            Some(x) => Some(x % self.mod_by),
            None => None,
        }
    }
}

impl<I: Source<Item = f32>> Source for RawMod<I> {
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
