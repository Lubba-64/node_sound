use crate::sound_map::SoundNode;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Avg<I: SoundNode> {
    source: I,
    table: VecDeque<f32>,
    size: usize,
}

impl<I: SoundNode> Avg<I> {
    #[inline]
    pub fn new(source: I, table_size: usize) -> Self {
        Self {
            source,
            table: VecDeque::new(),
            size: table_size,
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Avg<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.table
            .push_back(self.source.next(index, channel).unwrap_or_default());
        if self.table.len() > self.size {
            self.table.pop_front();
        }
        Some(self.table.iter().map(|f| *f).sum::<f32>() / self.table.len() as f32)
    }
}
