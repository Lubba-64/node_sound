use crate::node::SoundNode;

#[derive(Clone, Debug)]
pub struct Speed<I: SoundNode> {
    source: I,
    speed: f32,
}

impl<I: SoundNode> Speed<I> {
    pub fn new(source: I, speed: f32) -> Self {
        Self { source, speed }
    }
}

impl<I: SoundNode + Clone> SoundNode for Speed<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let scaled_index = index * self.speed;
        self.source.next(scaled_index, channel)
    }
}
