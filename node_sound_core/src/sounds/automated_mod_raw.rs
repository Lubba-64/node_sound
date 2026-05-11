use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct AutomatedModRaw<I1: SoundNode, I2: SoundNode> {
    source: I1,
    mod_by: I2,
}

impl<I1: SoundNode, I2: SoundNode> AutomatedModRaw<I1, I2> {
    #[inline]
    pub fn new(source: I1, mod_by: I2) -> Self {
        Self { source, mod_by }
    }
}

impl<I1: SoundNode + Clone, I2: SoundNode + Clone> SoundNode for AutomatedModRaw<I1, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match (
            self.source.next(index, channel),
            self.mod_by.next(index, channel),
        ) {
            (Some(sample), Some(mod_by)) => Some(sample % mod_by),
            _ => None,
        }
    }
}
