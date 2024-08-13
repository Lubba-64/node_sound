use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct Wrapper<I: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
    last: Option<f32>,
}

impl<I: Source<Item = f32>> Wrapper<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
            last: None,
        }
    }
}

impl<I: Source<Item = f32>> Iterator for Wrapper<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.last = match (self.source.next(), self.last) {
            (Some(x), Some(y)) => {
                if x + y > 1.0 {
                    return Some(-1.0);
                }
                Some(x + y)
            }
            (None, Some(y)) => Some(y),
            (Some(x), None) => Some(x),
            _ => Some(0.0),
        };
        Some(self.last.unwrap_or(0.0))
    }
}

impl<I: Source<Item = f32>> Source for Wrapper<I> {
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
