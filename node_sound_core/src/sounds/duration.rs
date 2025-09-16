use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::DawSource};

#[derive(Clone)]
pub struct Duration<I: DawSource> {
    source: I,
    duration: f32,
    sample_rate: f32,
    speed: f32,
    uses_speed: bool,
}

impl<I: DawSource + Clone> Duration<I> {
    #[inline]
    pub fn new(source: I, duration: f32, uses_speed: bool) -> Self {
        Self {
            source,
            duration,
            sample_rate: DEFAULT_SAMPLE_RATE as f32,
            speed: 1.0,
            uses_speed,
        }
    }
}

impl<I: DawSource + Clone> DawSource for Duration<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if index > self.sample_rate * self.duration * if self.uses_speed { self.speed } else { 1.0 }
        {
            None
        } else {
            self.source.next(index, channel)
        }
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.source.note_speed(speed, rate);
        self.sample_rate = rate;
        if self.uses_speed {
            self.speed = speed;
        }
    }
    fn size_hint(&self) -> Option<f32> {
        Some(self.duration)
    }
}
