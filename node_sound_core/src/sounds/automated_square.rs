use std::f32::consts::PI;
use std::time::Duration;

use rodio::source::UniformSourceIterator;
use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;

#[derive(Clone)]
pub struct AutomatedSquareWave<T: rodio::Source<Item = f32>> {
    freq: UniformSourceIterator<T, f32>,
    num_sample: usize,
}

impl<T: rodio::Source<Item = f32>> AutomatedSquareWave<T> {
    #[inline]
    pub fn new(freq: T) -> AutomatedSquareWave<T> {
        AutomatedSquareWave {
            freq: UniformSourceIterator::new(freq, 1, DEFAULT_SAMPLE_RATE),
            num_sample: 0,
        }
    }
}

impl<T: rodio::Source<Item = f32>> Iterator for AutomatedSquareWave<T> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        self.freq.next().map(|freq| {
            let value = 2.0
                * PI
                * freq
                * (self.num_sample as f32 % DEFAULT_SAMPLE_RATE as f32)
                / DEFAULT_SAMPLE_RATE as f32;
            value.sin().signum()
        })
    }
}

impl<T: rodio::Source<Item = f32>> Source for AutomatedSquareWave<T> {
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
