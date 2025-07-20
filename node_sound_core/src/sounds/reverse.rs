use rodio::Source;

use crate::constants::DEFAULT_SAMPLE_RATE;
use rodio::source::UniformSourceIterator;
use std::time::Duration;

#[derive(Clone)]
pub struct ReverseSource {
    buffer: Vec<f32>,
    idx: usize,
}

impl ReverseSource {
    #[inline]
    pub fn new(source: impl Source<Item = f32>, duration: Duration) -> Self {
        let len = (duration.as_secs_f32() * DEFAULT_SAMPLE_RATE as f32).round() as usize * 2;
        let mut uniform = UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE);
        Self {
            buffer: (0..len).map(|_| uniform.next().unwrap_or(0.0)).collect(),
            idx: 0,
        }
    }
}

impl Iterator for ReverseSource {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        if self.idx >= self.buffer.len() {
            None
        } else {
            let sample = self.buffer[self.buffer.len() - self.idx - 1];
            self.idx += 1;
            Some(sample)
        }
    }
}

impl Source for ReverseSource {
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
