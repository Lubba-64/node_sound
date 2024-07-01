use rodio::{source::UniformSourceIterator, Source};

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use std::time::Duration;

#[derive(Clone)]
pub struct AutomatedSawToothWave<T: rodio::Source<Item = f32>> {
    freq: UniformSourceIterator<T, f32>,
    num_sample: usize,
}

impl<T: rodio::Source<Item = f32>> AutomatedSawToothWave<T> {
    #[inline]
    pub fn new(freq: T) -> Self {
        Self {
            freq: UniformSourceIterator::new(freq, 1, DEFAULT_SAMPLE_RATE),
            num_sample: 0,
        }
    }
}

impl<T: rodio::Source<Item = f32>> Iterator for AutomatedSawToothWave<T> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        let freq = self.freq.next().unwrap_or(0.0);

        let value = (freq * (self.num_sample as f32 % DEFAULT_SAMPLE_RATE as f32))
            / DEFAULT_SAMPLE_RATE as f32;
        Some(value % 1.0)
    }
}

impl<T: rodio::Source<Item = f32>> Source for AutomatedSawToothWave<T> {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
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
