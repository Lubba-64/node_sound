use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct Skip<S: SoundNode> {
    duration: f32,
    source: S,
    sample_rate: f32,
    speed: f32,
}

impl<S: SoundNode> Skip<S> {
    pub fn new(duration: f32, source: S, uses_speed: bool, sample_rate: f32, speed: f32) -> Self {
        Self {
            duration,
            source,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl<S: SoundNode + Clone> SoundNode for Skip<S> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index += self.duration * self.speed * self.sample_rate;
        self.source.next(index, channel)
    }
}
