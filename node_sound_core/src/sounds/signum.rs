use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct Signum<I: SoundNode> {
    source: I,
}

impl<I: SoundNode> Signum<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self { source }
    }
}

impl<I: SoundNode + Clone> SoundNode for Signum<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source
            .next(index, channel)
            .map(|sample| sample.signum())
    }
}
