use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct Clamp<I: DawSource> {
    source: I,
    min: f32,
    max: f32,
}

impl<I: DawSource> Clamp<I> {
    #[inline]
    pub fn new(source: I, min: Option<f32>, max: Option<f32>) -> Self {
        let mut min_1 = min.unwrap_or(f32::MIN);
        let mut max_1 = max.unwrap_or(f32::MAX);
        if min_1 > max_1 {
            let other = min_1;
            min_1 = max_1;
            max_1 = other;
        }
        Self {
            source,
            max: max_1,
            min: min_1,
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
