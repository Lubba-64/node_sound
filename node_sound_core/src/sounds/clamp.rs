use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct Clamp<I: DawSource> {
    source: I,
    min: f32,
    max: f32,
}

impl<I: DawSource> Clamp<I> {
    #[inline]
    pub fn new(source: I, mut min: f32, mut max: f32) -> Self {
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }
        Self {
            source,
            max: max,
            min: min,
        }
    }
}

impl<I: DawSource + Clone> DawSource for Clamp<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        return self
            .source
            .next(index, channel)
            .map(|val| val.clamp(self.min, self.max));
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.source.note_speed(speed, rate);
    }
    fn size_hint(&self) -> Option<f32> {
        self.source.size_hint()
    }
}
