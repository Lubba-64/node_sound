use crate::sound_map::DawSource;

#[derive(Clone, Debug)]
pub struct AutomatedTranslateWave<
    I1: DawSource,
    I2: DawSource,
    I3: DawSource,
    I4: DawSource,
    I5: DawSource,
> {
    source: I1,
    start_min: I2,
    start_max: I3,
    end_min: I4,
    end_max: I5,
}

impl<I1: DawSource, I2: DawSource, I3: DawSource, I4: DawSource, I5: DawSource>
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
    I1: DawSource + Clone,
    I2: DawSource + Clone,
    I3: DawSource + Clone,
    I4: DawSource + Clone,
    I5: DawSource + Clone,
> DawSource for AutomatedTranslateWave<I1, I2, I3, I4, I5>
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
    fn size_hint(&self) -> Option<f32> {
        let end_max = self.end_max.size_hint()?;
        let end_min = self.end_min.size_hint()?;
        let start_max = self.start_max.size_hint()?;
        let start_min = self.start_min.size_hint()?;
        let source = self.source.size_hint()?;
        Some(end_max.max(end_min.max(start_max.max(start_min.max(source)))))
    }
}
