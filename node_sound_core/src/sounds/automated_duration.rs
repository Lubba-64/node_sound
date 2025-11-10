use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct AutomatedDuration<I: DawSource, D: DawSource> {
    source: I,
    duration: D,
    sample_rate: f32,
    speed: f32,
}

impl<S: DawSource, D: DawSource> AutomatedDuration<S, D> {
    pub fn new(duration: D, source: S, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            duration,
            source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl<I: DawSource + Clone, D: DawSource + Clone> DawSource for AutomatedDuration<I, D> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if index / self.speed
            > self.sample_rate * self.duration.next(index, channel).unwrap_or_default()
        {
            None
        } else {
            self.source.next(index, channel)
        }
    }
}
