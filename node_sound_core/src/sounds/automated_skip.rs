use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct AutomatedSkip<S: DawSource, D: DawSource> {
    duration: D,
    source: S,
    sample_rate: f32,
    speed: f32,
}

impl<S: DawSource, D: DawSource> AutomatedSkip<S, D> {
    pub fn new(duration: D, source: S, uses_speed: bool, sample_rate: f32, speed: f32) -> Self {
        Self {
            duration,
            source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl<S: DawSource + Clone, D: DawSource + Clone> DawSource for AutomatedSkip<S, D> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index +=
            self.duration.next(index, channel).unwrap_or_default() * self.speed * self.sample_rate;
        self.source.next(index, channel)
    }
}
