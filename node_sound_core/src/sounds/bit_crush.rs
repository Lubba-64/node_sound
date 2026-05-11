use crate::node::SoundNode;

#[derive(Clone, Debug)]
pub struct BitCrusher<I: SoundNode> {
    source: I,
    step_size: f32,
}

impl<I: SoundNode> BitCrusher<I> {
    #[inline]
    pub fn new(source: I, bits: u32) -> Self {
        let bits = bits.clamp(1, 16);
        let step_size = 1.0 / bits as f32;
        Self { source, step_size }
    }
}

impl<I: SoundNode + Clone> SoundNode for BitCrusher<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source
            .next(index, channel)
            .map(|sample| ((sample / self.step_size).rem_euclid(self.step_size)).clamp(-1.0, 1.0))
    }
}
