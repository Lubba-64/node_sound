use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct Mod<I: DawSource> {
    source: I,
    mod_by: f32,
}

impl<I: DawSource> Mod<I> {
    #[inline]
    pub fn new(source: I, mod_by: f32) -> Self {
        Self { source, mod_by }
    }
}

impl<I: DawSource + Clone> DawSource for Mod<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match self.source.next(index, channel) {
            Some(x) => Some(x % self.mod_by),
            None => None,
        }
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.source.note_speed(speed, rate);
    }
    fn size_hint(&self) -> Option<f32> {
        self.source.size_hint()
    }
}
