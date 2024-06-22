use std::f32::consts::PI;
use std::time::Duration;

use rodio::source::UniformSourceIterator;
use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;

#[derive(Clone)]
pub struct AutomatedSineWave<T: rodio::Source<Item = f32>> {
    freq: UniformSourceIterator<T, f32>,
    num_sample: usize,
}

impl<T: rodio::Source<Item = f32>> AutomatedSineWave<T> {
    #[inline]
    pub fn new(freq: T) -> AutomatedSineWave<T> {
        AutomatedSineWave {
            freq: UniformSourceIterator::new(freq, 1, DEFAULT_SAMPLE_RATE),
            num_sample: 0,
        }
    }
}

impl<T: rodio::Source<Item = f32>> Iterator for AutomatedSineWave<T> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        let value = 2.0 * PI * self.freq.next().unwrap_or(0.0) * self.num_sample as f32 / 48000.0;
        Some(value.sin())
    }
}

impl<T: rodio::Source<Item = f32>> Source for AutomatedSineWave<T> {
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
