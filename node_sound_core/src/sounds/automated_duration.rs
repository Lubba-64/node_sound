use crate::node::SoundNode;

#[derive(Clone, Debug)]
pub struct AutomatedDuration<I: SoundNode, D: SoundNode> {
    source: I,
    duration: D,
    sample_rate: f32,
    speed: f32,
}

impl<S: SoundNode, D: SoundNode> AutomatedDuration<S, D> {
    pub fn new(duration: D, source: S, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            duration,
            source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl<I: SoundNode + Clone, D: SoundNode + Clone> SoundNode for AutomatedDuration<I, D> {
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
