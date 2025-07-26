use crate::constants::DEFAULT_SAMPLE_RATE;
use eframe::egui::mutex::Mutex;
use rodio::Source;
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub struct DawAutomationChannel {
    channel: Arc<Mutex<f32>>,
}

impl DawAutomationChannel {
    #[inline]
    pub fn new(channel: Arc<Mutex<f32>>) -> Self {
        Self { channel }
    }
}

impl Iterator for DawAutomationChannel {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        Some(*self.channel.lock())
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
