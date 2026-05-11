use crate::node::SoundNode;

#[derive(Clone, Debug)]
pub struct SplitChannels<I: SoundNode> {
    source: I,
    channel: u8,
}

impl<I: SoundNode> SplitChannels<I> {
    #[inline]
    pub fn new(source: I, channel: u8) -> Self {
        Self { source, channel }
    }
}

impl<I: SoundNode + Clone> SoundNode for SplitChannels<I> {
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
        self.source.next(index, self.channel)
    }
}
