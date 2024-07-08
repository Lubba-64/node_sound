use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use std::time::Duration;

#[derive(Clone)]
pub struct SamplesSource {
    source: Vec<f32>,
    idx: usize,
}

impl SamplesSource {
    #[inline]
    pub fn new(source: Vec<f32>) -> Self {
        Self {
            source: source,
            idx: 0,
        }
    }
}

impl Iterator for SamplesSource {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.idx += 1;
        if self.idx < self.source.len() {
            Some(self.source[self.idx])
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        return (self.source.len(), Some(self.source.len()))
    }
}

impl Source for SamplesSource {
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
