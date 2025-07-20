use rodio::Source;

use crate::constants::DEFAULT_SAMPLE_RATE;
use std::time::Duration;

pub static mut DAW_INPUT: Option<(u32, f32)> = None;

#[derive(Clone)]
pub struct DawInputChannel {}

impl DawInputChannel {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl Iterator for DawInputChannel {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        unsafe {
            match DAW_INPUT.as_ref() {
                None => None,
                Some(x) => Some(x.1),
            }
        }
    }
}

impl Source for DawInputChannel {
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
        unsafe {
            match DAW_INPUT.as_ref() {
                Some(x) => x.0,
                None => DEFAULT_SAMPLE_RATE,
            }
        }
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
