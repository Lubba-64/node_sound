use crate::sound_map::DawSource;

#[derive(Clone)]
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
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
        Some(self.val)
    }

    fn note_speed(&mut self, _speed: f32) {}

    fn set_sample_rate(&mut self, _rate: f32) {}
}
