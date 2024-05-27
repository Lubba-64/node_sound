use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use rand::prelude::*;
use std::time::Duration;

#[derive(Clone)]
pub struct Pop {
    amplitude: f32,
    done: bool,
}

impl Pop {
    #[inline]
    pub fn new(amplitude: f32) -> Self {
        Self {
            amplitude: amplitude,
            done: false,
        }
    }
}

impl Iterator for Pop {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        if !self.done {
            self.done = true;
            return Some(self.amplitude);
        }
        Some(0.0)
    }
}

impl Source for Pop {
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
