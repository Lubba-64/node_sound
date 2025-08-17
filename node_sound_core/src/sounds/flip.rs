use rodio::Source;

use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::SetSpeed};
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct Flip<I: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
}

impl<I: Source<Item = f32>> Flip<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
        }
    }
}

impl<I: Source<Item = f32>> Iterator for Flip<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        match self.source.next() {
            Some(x) => Some(-x),
            None => None,
        }
    }
}

impl<I: Source<Item = f32>> Source for Flip<I> {
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

impl<I: Source<Item = f32>> SetSpeed<f32> for Flip<I> {
    fn set_speed(&mut self, _speed: f32) {}
}
