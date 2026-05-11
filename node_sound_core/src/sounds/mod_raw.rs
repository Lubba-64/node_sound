use crate::node::SoundNode;

#[derive(Clone, Debug)]
pub struct RawMod<I: SoundNode> {
    source: I,
    mod_by: f32,
}

impl<I: SoundNode> RawMod<I> {
    #[inline]
    pub fn new(source: I, mod_by: f32) -> Self {
        Self { source, mod_by }
    }
}

impl<I: SoundNode + Clone> SoundNode for RawMod<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match self.source.next(index, channel) {
            Some(sample) => Some(sample % self.mod_by),
            None => None,
        }
    }
}
