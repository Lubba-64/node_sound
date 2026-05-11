use crate::node::SoundNode;

#[derive(Clone, Debug)]
pub struct AutomatedTranslateWave<
    I1: SoundNode,
    I2: SoundNode,
    I3: SoundNode,
    I4: SoundNode,
    I5: SoundNode,
> {
    source: I1,
    start_min: I2,
    start_max: I3,
    end_min: I4,
    end_max: I5,
}

impl<I1: SoundNode, I2: SoundNode, I3: SoundNode, I4: SoundNode, I5: SoundNode>
    AutomatedTranslateWave<I1, I2, I3, I4, I5>
{
    #[inline]
    pub fn new(source: I1, start_min: I2, start_max: I3, end_min: I4, end_max: I5) -> Self {
        Self {
            source,
            start_max,
            start_min,
            end_max,
            end_min,
        }
    }
}

impl<
    I1: SoundNode + Clone,
    I2: SoundNode + Clone,
    I3: SoundNode + Clone,
    I4: SoundNode + Clone,
    I5: SoundNode + Clone,
> SoundNode for AutomatedTranslateWave<I1, I2, I3, I4, I5>
{
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match (
            self.source.next(index, channel),
            self.start_min.next(index, channel),
            self.start_max.next(index, channel),
            self.end_min.next(index, channel),
            self.end_max.next(index, channel),
        ) {
            (
                Some(p),
                Some(mut start_min),
                Some(mut start_max),
                Some(mut end_min),
                Some(mut end_max),
            ) => {
                if start_min > start_max {
                    std::mem::swap(&mut start_min, &mut start_max);
                }
                if end_min > end_max {
                    std::mem::swap(&mut end_min, &mut end_max);
                }
                Some(
                    end_min
                        + ((end_max - end_min) / (start_max - start_min))
                            * (p.clamp(start_min, start_max) - start_min),
                )
            }
            _ => None,
        }
    }
}
