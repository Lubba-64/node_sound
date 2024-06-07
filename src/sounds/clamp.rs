use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct Clamp<I: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
    min: Option<f32>,
    max: Option<f32>,
}

impl<I: Source<Item = f32>> Clamp<I> {
    #[inline]
    pub fn new(source: I, min: Option<f32>, max: Option<f32>) -> Self {
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
            max: max,
            min: min,
        }
    }
}

impl<I: Source<Item = f32>> Iterator for Clamp<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        Some(
            self.source
                .next()
                .unwrap_or(0.0)
                .clamp(self.min.unwrap_or(f32::MIN), self.max.unwrap_or(f32::MAX)),
        )
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
