use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct Abs<I: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
}

impl<I: Source<Item = f32>> Abs<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
        }
    }
}

impl<I: Source<Item = f32>> Iterator for Abs<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        Some(self.source.next().map(|val| (val.abs() * 2.0) - 1.0))
    }
}

impl<I: Source<Item = f32>> Source for Abs<I> {
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
