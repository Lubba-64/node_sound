use crate::node::SoundNode;

#[derive(Clone, Debug)]
pub struct Duration<I: SoundNode> {
    source: I,
    duration: f32,
    sample_rate: f32,
    speed: f32,
}

impl<S: SoundNode> Duration<S> {
    pub fn new(duration: f32, source: S, uses_speed: bool, speed: f32, sample_rate: f32) -> Self {
        Self {
            duration,
            source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Duration<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if index / self.speed > self.sample_rate * self.duration {
            None
        } else {
            self.source.next(index, channel)
        }
    }
}
