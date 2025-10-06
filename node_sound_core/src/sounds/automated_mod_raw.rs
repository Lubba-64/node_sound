use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct AutomatedModRaw<I1: DawSource, I2: DawSource> {
    source: I1,
    mod_by: I2,
}

impl<I1: DawSource, I2: DawSource> AutomatedModRaw<I1, I2> {
    #[inline]
    pub fn new(source: I1, mod_by: I2) -> Self {
        Self { source, mod_by }
    }
}

impl<I1: DawSource + Clone, I2: DawSource + Clone> DawSource for AutomatedModRaw<I1, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match (
            self.source.next(index, channel),
            self.mod_by.next(index, channel),
        ) {
            (Some(x), Some(mod_by)) => Some(x % mod_by),
            _ => None,
        }
    }
}
