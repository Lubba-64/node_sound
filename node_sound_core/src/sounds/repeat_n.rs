use crate::sound_map::DawSource;
use std::u32;

#[derive(Clone, Debug)]
pub struct RepeatRefSource<I: DawSource> {
    source: I,
    repeat_count: Option<u32>,
    sample_rate: f32,
    ind_min: f32,
}

impl<I: DawSource + Clone> RepeatRefSource<I> {
    #[inline]
    pub fn new(source: I, repeat_count: Option<u32>, sample_rate: f32) -> Self {
        Self {
            source,
            repeat_count,
            sample_rate,
            ind_min: 0.0,
        }
    }
}

impl<I: DawSource + Clone> DawSource for RepeatRefSource<I> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        match self.source.size_hint() {
            None => {
                index -= self.ind_min;
                match self.source.next(index, channel) {
                    None => {
                        self.ind_min += index;
                        self.source.next(0.0, channel)
                    }
                    Some(x) => Some(x),
                }
            }
            Some(x) => {
                let wrap = x * self.sample_rate;

                match self.repeat_count {
                    None => {}
                    Some(repeat_count) => {
                        if (index / wrap) as u32 > repeat_count {
                            return None;
                        }
                    }
                }

                index %= wrap;
                self.source.next(index, channel)
            }
        }
    }
    fn size_hint(&self) -> Option<f32> {
        match self.repeat_count {
            None => None,
            Some(repeats) => match self.source.size_hint() {
                None => None,
                Some(size) => Some(size * repeats as f32),
            },
        }
    }
}
