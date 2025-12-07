use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct Glitch<I: DawSource> {
    current_source: I,
    ind_min: f32,
}

impl<I: DawSource + Clone> Glitch<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self {
            current_source: source,
            ind_min: 0.0,
        }
    }
}

impl<I: DawSource + Clone> DawSource for Glitch<I> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index += 0.1;
        if index > 0.1 + 0.1 {
            self.ind_min += index - 0.1;
        }
        index -= self.ind_min;
        self.current_source.next(index, channel)
    }
}
