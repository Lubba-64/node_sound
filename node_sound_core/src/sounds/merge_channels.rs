use crate::node::SoundNode;

#[derive(Clone, Debug)]
pub struct MergeChannels<I1: SoundNode, I2: SoundNode> {
    source1: I1,
    source2: I2,
}

impl<I1: SoundNode, I2: SoundNode> MergeChannels<I1, I2> {
    #[inline]
    pub fn new(source1: I1, source2: I2) -> Self {
        Self { source1, source2 }
    }
}

impl<I1: SoundNode + Clone, I2: SoundNode + Clone> SoundNode for MergeChannels<I1, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if channel == 0 {
            self.source1.next(index, 0)
        } else if channel == 1 {
            self.source2.next(index, 0)
        } else {
            None
        }
    }
}
