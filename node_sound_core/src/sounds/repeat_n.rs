use rodio::Source;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

use crate::sound_map::SetSpeed;

#[derive(Clone)]
pub struct RepeatRefSource<I: Source<Item = f32>> {
    source: UniformSourceIterator<I>,
    repeat_count: u32,
    current_repeat: u32,
    original_source: I,
}

impl<I: Source<Item = f32> + Clone> RepeatRefSource<I> {
    #[inline]
    pub fn new(source: I, repeat_count: u32) -> Self {
        Self {
            source: UniformSourceIterator::new(
                source.clone(),
                source.channels(),
                source.sample_rate(),
            ),
            repeat_count,
            current_repeat: 0,
            original_source: source,
        }
    }
}

impl<I: Source<Item = f32> + Clone> Iterator for RepeatRefSource<I> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        match self.source.next() {
            Some(sample) => Some(sample),
            None => {
                if self.current_repeat < self.repeat_count - 1 {
                    self.current_repeat += 1;
                    self.source = UniformSourceIterator::new(
                        self.original_source.clone(),
                        self.original_source.channels(),
                        self.original_source.sample_rate(),
                    );
                    self.source.next()
                } else {
                    None // Stop after reaching the repeat count
                }
            }
        }
    }
}

impl<I: Source<Item = f32> + Clone> Source for RepeatRefSource<I> {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        self.source.current_span_len()
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.source.channels()
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.original_source
            .total_duration()
            .map(|d| d * self.repeat_count)
    }
}

impl<I: Source<Item = f32> + Clone> SetSpeed for RepeatRefSource<I> {
    fn set_speed(&mut self, _speed: f32) {}
}
