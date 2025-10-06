use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct RawMod<I: DawSource> {
    source: I,
    mod_by: f32,
}

impl<I: DawSource> RawMod<I> {
    #[inline]
    pub fn new(source: I, mod_by: f32) -> Self {
        Self { source, mod_by }
    }
}

impl<I: DawSource + Clone> DawSource for RawMod<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match self.source.next(index, channel) {
            Some(x) => Some(x % self.mod_by),
            None => None,
        }
    }
}
