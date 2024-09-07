use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct SawToothWave {
    freq: f32,
    num_sample: usize,
}

impl SawToothWave {
    #[inline]
    pub fn new(freq: f32) -> Self {
        Self {
            freq,
            num_sample: 0,
        }
    }
}

impl Iterator for SawToothWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = (self.freq * self.num_sample as f32) / DEFAULT_SAMPLE_RATE as f32;
        Some((value % 2.0) - 1.0)
    }
}

impl Source for SawToothWave {
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
