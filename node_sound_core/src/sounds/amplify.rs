use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct Amplify<I: DawSource> {
    source: I,
    amplification: f32,
}

impl<I: DawSource> Amplify<I> {
    #[inline]
    pub fn new(source: I, amplification: f32) -> Self {
        Self {
            source,
            amplification,
        }
    }
}

impl<I: DawSource + Clone> DawSource for Amplify<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source
            .next(index, channel)
            .map(|x| x * self.amplification)
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.source.note_speed(speed, rate);
    }
    fn size_hint(&self) -> Option<f32> {
        self.source.size_hint()
    }
}
