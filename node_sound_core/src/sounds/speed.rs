use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct Speed<I: DawSource> {
    source: I,
    speed: f32,
}

impl<I: DawSource> Speed<I> {
    pub fn new(source: I, speed: f32) -> Self {
        Self { source, speed }
    }
}

impl<I: DawSource + Clone> DawSource for Speed<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let scaled_index = index * self.speed;
        self.source.next(scaled_index, channel)
    }
}
