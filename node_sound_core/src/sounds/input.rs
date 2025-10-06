use crate::sound_map::DawSource;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct InputChannel {
    channel: Arc<Mutex<(f32, f32)>>,
}

impl InputChannel {
    #[inline]
    pub fn new(channel: Arc<Mutex<(f32, f32)>>) -> Self {
        Self { channel }
    }
}

impl DawSource for InputChannel {
    fn next(&mut self, _index: f32, channel: u8) -> Option<f32> {
        match self.channel.lock() {
            Err(_x) => None,
            Ok(x) => Some(if channel == 0 { x.0 } else { x.1 }),
        }
    }
}
