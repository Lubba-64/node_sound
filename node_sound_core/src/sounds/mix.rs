use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct Mix<I1: DawSource, I2: DawSource> {
    source1: I1,
    source2: I2,
}

impl<I1: DawSource, I2: DawSource> Mix<I1, I2> {
    #[inline]
    pub fn new(source1: I1, source2: I2) -> Self {
        Self { source1, source2 }
    }
}

impl<I1: DawSource + Clone, I2: DawSource + Clone> DawSource for Mix<I1, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let s1 = self.source1.next(index, channel)?;
        let s2 = self.source2.next(index, channel)?;
        Some((s1 + s2) / 2.0)
    }

    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.source1.note_speed(speed, rate);
        self.source2.note_speed(speed, rate);
    }
}
