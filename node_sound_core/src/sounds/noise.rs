use crate::sound_map::DawSource;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct Noise {
    min: f32,
    max: f32,
}

impl Noise {
    #[inline]
    pub fn new(min: f32, max: f32) -> Self {
        let mut min_1 = min;
        let mut max_1 = max;
        if min_1 > max_1 {
            let other = min_1;
            min_1 = max_1;
            max_1 = other;
        }
        Self {
            min: min_1,
            max: max_1,
        }
    }
}

impl DawSource for Noise {
    fn next(&mut self, _index: f32, _channel: u8) -> Option<f32> {
        if self.min == self.max {
            return Some(self.min);
        }
        Some(thread_rng().gen_range(self.min..self.max))
    }
    fn size_hint(&self) -> Option<f32> {
        None
    }
}
