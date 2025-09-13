use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct Flip<I1: DawSource> {
    source: I1,
}

impl<I1: DawSource> Flip<I1> {
    #[inline]
    pub fn new(source: I1) -> Self {
        Self { source }
    }
}

impl<I1: DawSource + Clone> DawSource for Flip<I1> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel).map(|x| -x)
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.source.note_speed(speed, rate);
    }
    fn size_hint(&self) -> Option<f32> {
        self.source.size_hint()
    }
}
