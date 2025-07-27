use rodio::Source;

use crate::constants::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct Clamp<I: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
    min: f32,
    max: f32,
}

impl<I: Source<Item = f32>> Clamp<I> {
    #[inline]
    pub fn new(source: I, min: Option<f32>, max: Option<f32>) -> Self {
        let mut min_1 = min.unwrap_or(f32::MIN);
        let mut max_1 = max.unwrap_or(f32::MAX);
        if min_1 > max_1 {
            let other = min_1;
            min_1 = max_1;
            max_1 = other;
        }
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
            max: max_1,
            min: min_1,
        }
    }
}

impl<I: Source<Item = f32>> Iterator for Clamp<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        return self.source.next().map(|val| val.clamp(self.min, self.max));
    }
}

impl<I: Source<Item = f32>> Source for Clamp<I> {
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
