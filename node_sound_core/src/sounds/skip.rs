use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::DawSource};

#[derive(Clone)]
pub struct Skip<S: DawSource> {
    duration: f32,
    source: S,
    sample_rate: f32,
    uses_speed: bool,
    speed: f32,
}

impl<S: DawSource> Skip<S> {
    pub fn new(duration: f32, source: S, uses_speed: bool) -> Self {
        Self {
            duration,
            source,
            speed: 1.0,
            sample_rate: DEFAULT_SAMPLE_RATE as f32,
            uses_speed,
        }
    }
}

impl<S: DawSource + Clone> DawSource for Skip<S> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index += self.duration * self.speed * self.sample_rate;
        self.source.next(index, channel)
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.sample_rate = rate;
        if self.uses_speed {
            self.speed = speed;
        }
        self.source.note_speed(speed, rate);
    }
    fn size_hint(&self) -> Option<f32> {
        self.source
            .size_hint()
            .map(|x| x - self.duration * self.speed)
    }
}
