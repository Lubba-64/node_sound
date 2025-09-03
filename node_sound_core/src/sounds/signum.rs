use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct Signum<I: DawSource> {
    source: I,
}

impl<I: DawSource> Signum<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self { source }
    }
}

impl<I: DawSource> DawSource for Signum<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel).map(|x| x.signum())
    }
    fn note_speed(&mut self, _speed: f32) {}
    fn set_sample_rate(&mut self, _rate: f32) {}
}
