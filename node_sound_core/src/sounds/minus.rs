use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct Minus<I1: SoundNode, I2: SoundNode> {
    source1: I1,
    source2: I2,
}

impl<I1: SoundNode, I2: SoundNode> Minus<I1, I2> {
    #[inline]
    pub fn new(source1: I1, source2: I2) -> Self {
        Self { source1, source2 }
    }
}

impl<I1: SoundNode + Clone, I2: SoundNode + Clone> SoundNode for Minus<I1, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match (
            self.source2.next(index, channel),
            self.source1.next(index, channel),
        ) {
            (Some(sample1), Some(sample2)) => Some(sample1 - sample2),
            _ => None,
        }
    }
}
