use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct Lfo<I1: DawSource, I2: DawSource> {
    source1: I1,
    source2: I2,
}

impl<I1: DawSource, I2: DawSource> Lfo<I1, I2> {
    #[inline]
    pub fn new(source1: I1, source2: I2) -> Self {
        Self { source1, source2 }
    }
}

impl<I1: DawSource + Clone, I2: DawSource + Clone> DawSource for Lfo<I1, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match (
            self.source2.next(index, channel),
            self.source1.next(index, channel),
        ) {
            (Some(x), Some(y)) => Some(x * y),
            _ => None,
        }
    }
    fn note_speed(&mut self, _speed: f32, _rate: f32) {}
}
