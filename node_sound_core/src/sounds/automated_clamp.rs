use crate::sound_map::SoundNode;

#[derive(Clone, Debug)]
pub struct AutomatedClamp<I1: SoundNode, I2: SoundNode, I3: SoundNode> {
    source: I1,
    min: I2,
    max: I3,
}

impl<I1: SoundNode, I2: SoundNode, I3: SoundNode> AutomatedClamp<I1, I2, I3> {
    #[inline]
    pub fn new(source: I1, min: I2, max: I3) -> Self {
        Self { source, max, min }
    }
}

impl<I1: SoundNode + Clone, I2: SoundNode + Clone, I3: SoundNode + Clone> SoundNode
    for AutomatedClamp<I1, I2, I3>
{
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match (
            self.source.next(index, channel),
            self.min.next(index, channel),
            self.max.next(index, channel),
        ) {
            (Some(source), Some(mut min), Some(mut max)) => {
                if min > max {
                    std::mem::swap(&mut min, &mut max);
                }
                Some(source.clamp(min, max))
            }
            _ => None,
        }
    }
}
