use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use crate::sounds::impls::StaticSource;
use crate::sounds::AsGenericSource;
use rodio::source::Zero;
use rodio::Source;
use std::time::Duration;

use super::GenericSource;

#[derive(Clone)]
pub struct AdvancedDelay {
    source: GenericSource<f32>,
}

impl AdvancedDelay {
    #[inline]
    pub fn new<S: StaticSource<f32>>(
        source: S,
        delays: u32,
        attenuation: f32,
        falloff: f32,
    ) -> Self {
        let mut amplitude = 1.0f32;
        let mut mixin: GenericSource<f32> = Zero::new(1, DEFAULT_SAMPLE_RATE).as_generic(None);
        for i in 0..delays {
            amplitude -= falloff;
            if amplitude < 0.0 {
                break;
            }
            mixin = mixin
                .mix(
                    source
                        .clone()
                        .delay(Duration::from_secs_f32(i as f32 * attenuation))
                        .amplify(amplitude),
                )
                .as_generic(None)
        }
        Self { source: mixin }
    }
}

impl Iterator for AdvancedDelay {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        Some(self.source.next().unwrap_or(0.0))
    }
}

impl Source for AdvancedDelay {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        DEFAULT_SAMPLE_RATE
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
