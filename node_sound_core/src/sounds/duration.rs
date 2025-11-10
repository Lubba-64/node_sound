use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct Duration<I: DawSource> {
    source: I,
    duration: f32,
    sample_rate: f32,
    speed: f32,
}

impl<S: DawSource> Duration<S> {
    pub fn new(duration: f32, source: S, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            duration,
            source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl<I: DawSource + Clone> DawSource for Duration<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if index / self.speed > self.sample_rate * self.duration {
            None
        } else {
            self.source.next(index, channel)
        }
    }
}
