pub static mut DAW_BUFF: [Option<f32>; 18] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None,
];

use rodio::Source;

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use std::time::Duration;

#[derive(Clone)]
pub struct DawAutomationChannel {
    channel: u8,
}

impl DawAutomationChannel {
    #[inline]
    pub fn new(channel: u8) -> Self {
        Self { channel }
    }
}

impl Iterator for DawAutomationChannel {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        match unsafe { &DAW_BUFF[self.channel.clamp(0, 17) as usize] } {
            None => None,
            Some(x) => Some(x.clone()),
        }
    }
}

impl Source for DawAutomationChannel {
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
