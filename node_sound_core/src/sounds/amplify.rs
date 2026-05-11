use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct Amplify<I: SoundNode> {
    source: I,
    amplification: f32,
}

impl<I: SoundNode> Amplify<I> {
    #[inline]
    pub fn new(source: I, amplification: f32) -> Self {
        Self {
            source,
            amplification,
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Amplify<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source
            .next(index, channel)
            .map(|x| x * self.amplification)
    }
}
