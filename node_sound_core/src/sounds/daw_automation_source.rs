use crate::sound_map::DawSource;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug)]
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
}
