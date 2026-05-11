use crate::sound_map::SoundNode;
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

impl SoundNode for InputChannel {
    fn next(&mut self, _index: f32, channel: u8) -> Option<f32> {
        match self.channel.lock() {
            Err(_) => None,
            Ok(sample) => Some(if channel == 0 { sample.0 } else { sample.1 }),
        }
    }
}
