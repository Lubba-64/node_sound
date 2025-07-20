use rodio::Source;

use crate::constants::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct MergeChannels<I: Source<Item = f32>, I2: Source<Item = f32>> {
    source1: UniformSourceIterator<I, I::Item>,
    source2: UniformSourceIterator<I2, I2::Item>,
    flop: bool,
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>> MergeChannels<I, I2> {
    #[inline]
    pub fn new(source1: I, source2: I2) -> Self {
        Self {
            source1: UniformSourceIterator::new(source1, 1, DEFAULT_SAMPLE_RATE),
            source2: UniformSourceIterator::new(source2, 1, DEFAULT_SAMPLE_RATE),
            flop: false,
        }
    }
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>> Iterator for MergeChannels<I, I2> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.flop = !self.flop;

        match (self.flop, self.source1.next(), self.source2.next()) {
            (true, Some(sample), _) => Some(sample),
            (false, _, Some(sample)) => Some(sample),
            _ => None,
        }
    }
}

impl<I: Source<Item = f32>, I2: Source<Item = f32>> Source for MergeChannels<I, I2> {
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
