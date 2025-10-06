use rand::Rng;

use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct RandomDuration<I: DawSource> {
    source: I,
    duration_min: f32,
    duration_max: f32,
    duration: f32,
    sample_rate: f32,
    speed: f32,
    last_index: f32,
}

impl<I: DawSource + Clone> RandomDuration<I> {
    #[inline]
    pub fn new(
        source: I,
        duration_min: f32,
        duration_max: f32,
        uses_speed: bool,
        sample_rate: f32,
        speed: f32,
    ) -> Self {
        let mut _self = Self {
            source,
            duration_min,
            duration_max,
            sample_rate,
            speed: if uses_speed { speed } else { 1.0 },
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
        if index > self.sample_rate * self.duration * self.speed {
            None
        } else {
            self.source.next(index, channel)
        }
    }
}
