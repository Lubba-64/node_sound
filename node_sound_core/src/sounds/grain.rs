use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct Grain<I: DawSource, S: DawSource, L: DawSource> {
    current_source: I,
    ind_min: f32,
    start: S,
    len: L,
    sample_rate: f32,
    current_len: Option<f32>,
}

impl<I: DawSource + Clone, S: DawSource + Clone, L: DawSource + Clone> Grain<I, S, L> {
    #[inline]
    pub fn new(source: I, start: S, len: L, sample_rate: f32) -> Self {
        Self {
            current_source: source,
            ind_min: 0.0,
            start,
            len,
            sample_rate,
            current_len: None,
        }
    }
}

impl<I: DawSource + Clone, S: DawSource + Clone, L: DawSource + Clone> DawSource
    for Grain<I, S, L>
{
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        let start = self.start.next(index, channel)? * self.sample_rate;
        if self.current_len.is_none() {
            self.current_len = Some(self.len.next(index, channel)? * self.sample_rate);
        }
        index -= self.ind_min;
        if index > self.current_len.unwrap_or_default() {
            self.current_len = Some(self.len.next(index, channel)? * self.sample_rate);
            self.ind_min += index;
        }
        index += start;
        self.current_source.next(index, channel)
    }
}
