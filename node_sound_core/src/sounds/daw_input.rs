use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use std::time::Duration;

pub static mut DAW_INPUT: Option<(u32, Vec<f32>)> = None;

#[derive(Clone)]
pub struct DawInputChannel {
    idx: usize,
}

impl DawInputChannel {
    #[inline]
    pub fn new() -> Self {
        Self { idx: 0 }
    }
}

impl Iterator for DawInputChannel {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        unsafe {
            match DAW_INPUT.as_ref() {
                None => None,
                Some(x) => {
                    self.idx += 1;
                    if self.idx >= x.1.len() {
                        self.idx = 0;
                    }
                    Some(x.1[self.idx])
                }
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
