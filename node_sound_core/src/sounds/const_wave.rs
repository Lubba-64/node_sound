use crate::sound_map::DawSource;

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

impl DawSource for ConstWave {
    fn next(&mut self, _index: f32, _channel: u8) -> Option<f32> {
        Some(self.val)
    }
    fn size_hint(&self) -> Option<f32> {
        None
    }
}
