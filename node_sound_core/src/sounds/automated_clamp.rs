use rodio::Source;

use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::SetSpeed};
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
        match (self.source.next(), self.min.next(), self.max.next()) {
            (Some(source), Some(mut min), Some(mut max)) => {
                if min > max {
                    std::mem::swap(&mut min, &mut max);
                }
                Some(source.clamp(min, max))
            }
            _ => None,
        }
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

impl<I: Source<Item = f32>, I2: Source<Item = f32>, I3: Source<Item = f32>> SetSpeed<f32>
    for AutomatedClamp<I, I2, I3>
{
    fn set_speed(&mut self, _speed: f32) {}
}
