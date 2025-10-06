use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct Signum<I: DawSource> {
    source: I,
}

impl<I: DawSource> Signum<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self { source }
    }
}

impl<I: DawSource + Clone> DawSource for Signum<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel).map(|x| x.signum())
    }
}
