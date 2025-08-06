use rodio::Source;

use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::SetSpeed};
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct SplitChannels<I: Source<Item = f32>> {
    source: UniformSourceIterator<I>,
    channel: u16,
}

impl<I: Source<Item = f32>> SplitChannels<I> {
    #[inline]
    pub fn new(source: I, channel: u16) -> Self {
        let channels = source.channels();
        Self {
            source: UniformSourceIterator::new(source, channels, DEFAULT_SAMPLE_RATE),
            channel,
        }
    }
}

impl<I: Source<Item = f32>> Iterator for SplitChannels<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        if self.source.channels() - 1 < self.channel {
            return None;
        }
        for _ in 0..self.channel {
            let _ = self.source.next().is_none();
        }
        let item = self.source.next();
        for _ in self.channel..self.source.channels() - 1 {
            let _ = self.source.next().is_none();
        }
        return item;
    }
}

impl<I: Source<Item = f32>> Source for SplitChannels<I> {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
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

impl<I: Source<Item = f32>> SetSpeed for SplitChannels<I> {
    fn set_speed(&mut self, _speed: f32) {}
}
