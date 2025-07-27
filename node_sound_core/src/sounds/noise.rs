use rodio::Source;

use crate::constants::DEFAULT_SAMPLE_RATE;
use rand::prelude::*;
use std::time::Duration;

#[derive(Clone)]
pub struct Noise {
    min: f32,
    max: f32,
}

impl Noise {
    #[inline]
    pub fn new(min: f32, max: f32) -> Self {
        let mut min_1 = min;
        let mut max_1 = max;
        if min_1 > max_1 {
            let other = min_1;
            min_1 = max_1;
            max_1 = other;
        }
        Self {
            min: min_1,
            max: max_1,
        }
    }
}

impl Iterator for Noise {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        Some(thread_rng().gen_range(self.min..self.max))
    }
}

impl Source for Noise {
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
