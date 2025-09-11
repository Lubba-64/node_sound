use crate::sound_map::DawSource;
use std::sync::Arc;
use std::sync::Mutex;

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

impl DawSource for DawAutomationChannel {
    fn next(&mut self, _index: f32, _channel: u8) -> Option<f32> {
        match self.channel.lock() {
            Err(_x) => None,
            Ok(x) => Some(*x),
        }
    }
    fn note_speed(&mut self, _speed: f32, _rate: f32) {}
    fn size_hint(&self) -> Option<f32> {
        None
    }
}
