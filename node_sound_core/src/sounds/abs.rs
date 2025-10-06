use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct Abs<I: DawSource> {
    source: I,
}

impl<I: DawSource> Abs<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self { source }
    }
}

impl<I: DawSource + Clone> DawSource for Abs<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel).map(|x| x.abs())
    }
}
