use crate::{
    sound_map::DawSource,
    sounds::wave_table::{WaveTableManager, WaveTableOscillator},
};

#[derive(Clone, Debug)]
pub struct ReverseSource {
    wavetable: WaveTableOscillator,
}

impl ReverseSource {
    #[inline]
    pub fn new<S: DawSource>(
        source: S,
        duration: f32,
        sample_rate: f32,
        manager: &mut WaveTableManager,
    ) -> Self {
        Self {
            wavetable: manager.make_wavetable(
                sample_rate,
                1.0,
                source,
                duration,
                1.0,
                false,
                1.0,
                Box::new(|source, total_samples| {
                    let mut left = Vec::with_capacity(total_samples);
                    let mut right = Vec::with_capacity(total_samples);
                    for i in 0..total_samples {
                        let index = i as f32;
                        left.push(source.next(index, 0).unwrap_or(0.0));
                        right.push(source.next(index, 1).unwrap_or(0.0));
                    }
                    left.reverse();
                    right.reverse();
                    (left, right)
                }),
            ),
        }
    }
}

impl DawSource for ReverseSource {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.wavetable.get_sample(index, channel)
    }
}
