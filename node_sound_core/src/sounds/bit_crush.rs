use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct BitCrusher<I: DawSource> {
    source: I,
    step_size: f32,
}

impl<I: DawSource> BitCrusher<I> {
    #[inline]
    pub fn new(source: I, bits: u32) -> Self {
        let bits = bits.clamp(1, 16);
        let step_size = 1.0 / bits as f32;
        Self { source, step_size }
    }
}

impl<I: DawSource + Clone> DawSource for BitCrusher<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source
            .next(index, channel)
            .map(|sample| ((sample / self.step_size).rem_euclid(self.step_size)).clamp(-1.0, 1.0))
    }
    fn size_hint(&self) -> Option<f32> {
        self.source.size_hint()
    }
}
