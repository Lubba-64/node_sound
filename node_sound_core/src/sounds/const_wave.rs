use crate::sound_map::{DawSource, Oscillator};

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
}

impl Oscillator for ConstWave {
    fn calculate_output(&self) -> f32 {
        self.val
    }
    fn get_frequency(&self) -> f32 {
        1.0
    }
    fn get_phase(&self) -> f32 {
        1.0
    }
    fn set_frequency(&mut self, _freq: f32) {}
    fn set_phase(&mut self, _phase: f32) {}
}
