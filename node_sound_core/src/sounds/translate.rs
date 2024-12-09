use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct TranslateWave<I: Source<Item = f32>> {
    source: UniformSourceIterator<I, I::Item>,
    start_min: f32,
    start_max: f32,
    end_min: f32,
    end_max: f32,
}

impl<I: Source<Item = f32>> TranslateWave<I> {
    #[inline]
    pub fn new(
        source: I,
        mut start_min: f32,
        mut start_max: f32,
        mut end_min: f32,
        mut end_max: f32,
    ) -> Self {
        if start_min > start_max {
            let other = start_min;
            start_min = start_max;
            start_max = other;
        }
        if end_min > end_max {
            let other = end_min;
            end_min = end_max;
            end_max = other;
        }
        Self {
            source: UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE),
            start_min,
            start_max,
            end_min,
            end_max,
        }
    }
}

impl<I: Source<Item = f32>> Iterator for TranslateWave<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        return match self.source.next() {
            Some(x) => Some(self.end_min
                + ((self.end_max - self.end_min) / (self.start_max - self.start_min))
                    * (x.clamp(self.start_min, self.start_max) - self.start_min)),
            _ => None,
        };
    }
}

impl<I: Source<Item = f32>> Source for TranslateWave<I> {
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
