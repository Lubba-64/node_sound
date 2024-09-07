use rodio::{source::UniformSourceIterator, Source};
use synthrs::{midi::MidiSong, synthesizer::make_samples_from_midi, wave};

use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use std::time::Duration;

#[derive(Clone)]
pub struct MidiRenderer {
    samples: Vec<f64>,
    num_sample: usize,
}

impl MidiRenderer {
    #[inline]
    pub fn new<I: Source<Item = f32>>(source: I, song: MidiSong) -> Self {
        let mut source = UniformSourceIterator::new(source, 2, DEFAULT_SAMPLE_RATE);
        let length = source
            .current_frame_len()
            .unwrap_or(DEFAULT_SAMPLE_RATE as usize) as f64
            / DEFAULT_SAMPLE_RATE as f64;
        let num_samples = (DEFAULT_SAMPLE_RATE as f64 * length).floor() as usize;
        let mut samples: Vec<f64> = Vec::with_capacity(num_samples);
        for _ in 0usize..num_samples {
            samples.push(source.next().unwrap_or(0.0) as f64);
        }
        let sampler = |frequency: f64| {
            wave::sampler(
                frequency,
                &samples,
                samples.len(),
                262.0,
                DEFAULT_SAMPLE_RATE as usize,
            )
        };
        let samples = make_samples_from_midi(sampler, DEFAULT_SAMPLE_RATE as usize, false, song)
            .expect("midi play failed");
        Self {
            samples,
            num_sample: 0,
        }
    }
}

impl Iterator for MidiRenderer {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        if self.num_sample >= self.samples.len() {
            self.num_sample = 0;
            None
        } else {
            Some(self.samples[self.num_sample] as f32)
        }
    }
}

impl Source for MidiRenderer {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        2
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
