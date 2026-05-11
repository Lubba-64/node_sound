use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct ConstWave {
    val: f32,
}

impl ConstWave {
    #[inline]
    pub fn new(val: f32) -> Self {
        Self { val }
    }
}

impl SoundNode for ConstWave {
    fn next(&mut self, _index: f32, _channel: u8) -> Option<f32> {
        Some(self.val)
    }
}
