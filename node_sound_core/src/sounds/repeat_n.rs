use crate::sound_map::DawSource;
use std::u32;

#[derive(Clone, Debug)]
pub struct RepeatRefSource<I: DawSource> {
    original_source: I,
    current_source: I,
    repeat_count: Option<u32>,
    repeats: u32,
    ind_min: f32,
}

impl<I: DawSource + Clone> RepeatRefSource<I> {
    #[inline]
    pub fn new(source: I, repeat_count: Option<u32>) -> Self {
        Self {
            current_source: source.clone(),
            original_source: source,
            repeat_count,
            ind_min: 0.0,
            repeats: 0,
        }
    }
}

impl<I: DawSource + Clone> DawSource for RepeatRefSource<I> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        if self.repeat_count.unwrap_or(u32::MAX) <= self.repeats {
            return None;
        }
        if self.ind_min > index {
            self.ind_min = 0.0;
        }
        index -= self.ind_min;
        match self.current_source.next(index, channel) {
            None => {
                self.current_source = self.original_source.clone();
                self.repeats += 1;
                self.ind_min += index;
                self.current_source.next(0.0, channel)
            }
            Some(x) => Some(x),
        }
    }
}
