use crate::sound_map::DawSource;

#[derive(Clone)]
pub struct MergeChannels<I1: DawSource, I2: DawSource> {
    source1: I1,
    source2: I2,
}

impl<I1: DawSource, I2: DawSource> MergeChannels<I1, I2> {
    #[inline]
    pub fn new(source1: I1, source2: I2) -> Self {
        Self { source1, source2 }
    }
}

impl<I1: DawSource + Clone, I2: DawSource + Clone> DawSource for MergeChannels<I1, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        if channel == 0 {
            self.source1.next(index, 0)
        } else if channel == 1 {
            self.source2.next(index, 0)
        } else {
            None
        }
    }
    fn size_hint(&self) -> Option<f32> {
        let s1 = self.source1.size_hint()?;
        let s2 = self.source2.size_hint()?;
        Some(s1.max(s2))
    }
}
