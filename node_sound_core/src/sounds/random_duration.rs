use rand::Rng;

use crate::{constants::DEFAULT_SAMPLE_RATE, sound_map::DawSource};

#[derive(Clone)]
pub struct RandomDuration<I: DawSource> {
    source: I,
    duration_min: f32,
    duration_max: f32,
    duration: f32,
    sample_rate: f32,
    speed: f32,
    uses_speed: bool,
    last_index: f32,
}

impl<I: DawSource + Clone> RandomDuration<I> {
    #[inline]
    pub fn new(source: I, duration_min: f32, duration_max: f32, uses_speed: bool) -> Self {
        let mut _self = Self {
            source,
            duration_min,
            duration_max,
            sample_rate: DEFAULT_SAMPLE_RATE as f32,
            speed: 1.0,
            uses_speed,
            duration: duration_min,
            last_index: 0.0,
        };
        _self.next_duration();
        _self
    }

    fn next_duration(&mut self) {
        self.duration = if self.duration_min == self.duration_max {
            self.duration_min
        } else {
            rand::thread_rng().gen_range(self.duration_min..self.duration_max)
        }
    }
}

impl<I: DawSource + Clone> DawSource for RandomDuration<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if index < self.last_index {
            self.next_duration();
        }
        self.last_index = index;
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
        None
    }
}
