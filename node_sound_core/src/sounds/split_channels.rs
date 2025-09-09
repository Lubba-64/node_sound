use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct SplitChannels<I: DawSource> {
    source: I,
    channel: u8,
}

impl<I: DawSource> SplitChannels<I> {
    #[inline]
    pub fn new(source: I, channel: u8) -> Self {
        Self { source, channel }
    }
}

impl<I: DawSource + Clone> DawSource for SplitChannels<I> {
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
        self.source.next(index, self.channel)
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.source.note_speed(speed, rate);
    }
}
