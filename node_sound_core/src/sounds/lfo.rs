use rodio::Source;

use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::SetSpeed};
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct Lfo<I1: Source<Item = f32>, I2: Source<Item = f32>> {
    input1: UniformSourceIterator<I1, I1::Item>,
    input2: UniformSourceIterator<I2, I2::Item>,
}

impl<I1: Source<Item = f32>, I2: Source<Item = f32>> Lfo<I1, I2> {
    #[inline]
    pub fn new(source1: I1, source2: I2) -> Self {
        Self {
            input1: UniformSourceIterator::new(source1, 2, DEFAULT_SAMPLE_RATE),
            input2: UniformSourceIterator::new(source2, 2, DEFAULT_SAMPLE_RATE),
        }
    }
}

impl<I1: Source<Item = f32>, I2: Source<Item = f32>> Iterator for Lfo<I1, I2> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        return match (self.input1.next(), self.input2.next()) {
            (Some(x), Some(y)) => Some(x * y),
            _ => None,
        };
    }
}

impl<I1: Source<Item = f32>, I2: Source<Item = f32>> Source for Lfo<I1, I2> {
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

impl<I1: Source<Item = f32>, I2: Source<Item = f32>> SetSpeed<f32> for Lfo<I1, I2> {
    fn set_speed(&mut self, _speed: f32) {}
}
