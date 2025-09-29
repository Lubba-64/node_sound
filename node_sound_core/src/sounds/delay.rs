use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct Delay<S: DawSource> {
    duration: f32,
    source: S,
    sample_rate: f32,
    speed: f32,
}

impl<S: DawSource> Delay<S> {
    pub fn new(duration: f32, source: S, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            duration,
            source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl<S: DawSource + Clone> DawSource for Delay<S> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if index > self.duration * self.speed * self.sample_rate {
            self.source.next(index, channel)
        } else {
            Some(0.0)
        }
    }
    fn size_hint(&self) -> Option<f32> {
        self.source
            .size_hint()
            .map(|x| x + self.duration * self.speed)
    }
}
