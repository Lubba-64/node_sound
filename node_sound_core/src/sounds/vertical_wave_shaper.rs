use crate::node::SoundNode;

#[derive(Clone, Debug)]
pub struct VerticalWaveShaper<I: SoundNode> {
    source: I,
    table: Vec<f32>,
}

impl<I: SoundNode> VerticalWaveShaper<I> {
    #[inline]
    pub fn new(source: I, mut table: Vec<f32>) -> Self {
        table = table.iter().map(|sample| (sample + 1.0) / 2.0).collect();
        Self { source, table }
    }
}

impl<I: SoundNode + Clone> SoundNode for VerticalWaveShaper<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel).map(|sample| {
            let real_idx = sample.abs().clamp(0.0, 1.0) * (self.table.len() - 1) as f32;
            let idx = real_idx.floor() as usize;
            let initial_weight = real_idx - idx as f32;
            let initial = self.table[idx] * initial_weight;
            let second = if idx + 1 >= self.table.len() {
                self.table[idx] + 0.001
            } else {
                self.table[idx + 1]
            } * (1.0 - initial_weight);
            (initial + second) * sample.signum()
        })
    }
}
