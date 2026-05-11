use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct Mix<I1: SoundNode, I2: SoundNode> {
    source1: I1,
    source2: I2,
}

impl<I1: SoundNode, I2: SoundNode> Mix<I1, I2> {
    #[inline]
    pub fn new(source1: I1, source2: I2) -> Self {
        Self { source1, source2 }
    }
}

impl<I1: SoundNode + Clone, I2: SoundNode + Clone> SoundNode for Mix<I1, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let s1 = self.source1.next(index, channel)?;
        let s2 = self.source2.next(index, channel)?;
        Some((s1 + s2) / 2.0)
    }
}
