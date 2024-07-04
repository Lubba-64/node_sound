use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct AutomatedClamp<I: Source<Item = f32>, I2: Source<Item = f32>, I3: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
    min: UniformSourceIterator<I2, I2::Item>,
    max: UniformSourceIterator<I3, I3::Item>,
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>, I3: Source<Item = f32>>
    AutomatedClamp<I, I2, I3>
{
    #[inline]
    pub fn new(source: I, min: I2, max: I3) -> Self {
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
            max: UniformSourceIterator::new(max, 1, DEFAULT_SAMPLE_RATE),
            min: UniformSourceIterator::new(min, 1, DEFAULT_SAMPLE_RATE),
        }
    }
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>, I3: Source<Item = f32>> Iterator
    for AutomatedClamp<I, I2, I3>
{
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let mut min = self.min.next().unwrap_or(f32::MIN);
        let mut max = self.max.next().unwrap_or(f32::MAX);
        if min > max {
            let other = min;
            min = max;
            max = other;
        }
        Some(self.source.next().unwrap_or(0.0).clamp(
            min,
            max,
        ))
    }
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>, I3: Source<Item = f32>> Source
    for AutomatedClamp<I, I2, I3>
{
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
