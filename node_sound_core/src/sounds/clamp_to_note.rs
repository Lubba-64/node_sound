use crate::{sound_graph::note::Pitch, sound_map::SoundNode};
use std::f32;

#[derive(Clone, Debug)]
pub struct ClampToNote<I: SoundNode> {
    source: I,
}

impl<I: SoundNode> ClampToNote<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self { source }
    }
}

impl<I: SoundNode + Clone> SoundNode for ClampToNote<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let val = self.source.next(index, channel).unwrap_or_default();
        let mut least_idx = 0;
        let mut least = f32::MAX;
        for (idx, pitch) in Pitch::ALL_FREQ.iter().enumerate() {
            let pitch_diff = (pitch - val).abs();
            if pitch_diff < least {
                least_idx = idx;
                least = pitch_diff;
            }
        }
        Some(Pitch::ALL[least_idx].match_freq())
    }
}
