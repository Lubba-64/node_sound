use crate::sound_map::DawSource;
use std::collections::VecDeque;
#[derive(Clone)]
pub struct Avg<I: DawSource> {
    source: I,
    table: VecDeque<f32>,
    size: usize,
}

impl<I: DawSource> Avg<I> {
    #[inline]
    pub fn new(source: I, table_size: usize) -> Self {
        Self {
            source,
            table: VecDeque::new(),
            size: table_size,
        }
    }
}

impl<I: DawSource + Clone> DawSource for Avg<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.table
            .push_back(self.source.next(index, channel).unwrap_or_default());
        if self.table.len() > self.size {
            self.table.pop_front();
        }
        Some(self.table.iter().map(|f| *f).sum::<f32>() / self.table.len() as f32)
    }
    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.source.note_speed(speed, rate);
    }
    fn size_hint(&self) -> Option<f32> {
        None
    }
}
