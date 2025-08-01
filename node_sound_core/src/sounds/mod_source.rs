use rodio::Source;

use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::SetSpeed};
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct Mod<I: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
    mod_by: f32,
}

impl<I: Source<Item = f32>> Mod<I> {
    #[inline]
    pub fn new(source: I, mod_by: f32) -> Self {
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
            mod_by,
        }
    }
}

impl<I: Source<Item = f32>> Iterator for Mod<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.source.next().map(|x| x - (x % self.mod_by))
    }
}

impl<I: Source<Item = f32>> Source for Mod<I> {
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

impl<I: Source<Item = f32>> SetSpeed<f32> for Mod<I> {
    fn set_speed(&mut self, _speed: f32) {}
}
