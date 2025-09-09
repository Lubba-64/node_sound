use crate::{
    constants::DEFAULT_SAMPLE_RATE, sound_map::DawSource, sounds::wave_table::WavetableOscillator,
};
use std::time::Duration;

#[derive(Clone)]
pub struct ReverseSource {
    wavetable: WavetableOscillator,
}

impl ReverseSource {
    #[inline]
    pub fn new<S: DawSource>(mut source: S, duration: Duration) -> Self {
        let sample_rate = DEFAULT_SAMPLE_RATE;
        let num_samples = (duration.as_secs_f32() * sample_rate as f32).round() as usize;

        let mut left_samples = Vec::with_capacity(num_samples);
        let mut right_samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let index = i as f32;
            left_samples.push(source.next(index, 0).unwrap_or(0.0));
            right_samples.push(source.next(index, 1).unwrap_or(0.0));
        }

        left_samples.reverse();
        right_samples.reverse();

        let wavetable =
            WavetableOscillator::new_stereo(left_samples, right_samples, sample_rate, 1.0, false);

        Self { wavetable }
    }

    #[inline]
    pub fn from_wavetable(
        left_table: Vec<f32>,
        right_table: Vec<f32>,
        sample_rate: u32,
        uses_speed: bool,
    ) -> Self {
        let wavetable =
            WavetableOscillator::new_stereo(left_table, right_table, sample_rate, 1.0, uses_speed);
        Self { wavetable }
    }
}

impl DawSource for ReverseSource {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.wavetable.next(index, channel)
    }

    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.wavetable.note_speed(speed, rate);
    }
}
