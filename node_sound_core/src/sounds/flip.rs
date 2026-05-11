use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct Flip<I1: SoundNode> {
    source: I1,
}

impl<I1: SoundNode> Flip<I1> {
    #[inline]
    pub fn new(source: I1) -> Self {
        Self { source }
    }
}

impl<I1: SoundNode + Clone> SoundNode for Flip<I1> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel).map(|sample| -sample)
    }
}
