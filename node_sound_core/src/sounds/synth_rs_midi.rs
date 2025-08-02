use crate::{
    constants::{DEFAULT_SAMPLE_RATE, MIDDLE_C_FREQ},
    sound_map::SetSpeed,
};
use rodio::{Source, source::UniformSourceIterator};
use std::time::Duration;
use synthrs::{midi::MidiSong, synthesizer::make_samples_from_midi, wave};

#[derive(Clone)]
pub struct MidiRenderer {
    midi_samples: Vec<f64>,
    num_sample: usize,
    speed: f32,
    uses_speed: bool,
    samples: Vec<f64>,
    song: MidiSong,
}

impl MidiRenderer {
    #[inline]
    pub fn new<I: Source<Item = f32>>(source: I, song: MidiSong, uses_speed: bool) -> Self {
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
                MIDDLE_C_FREQ as f64,
                DEFAULT_SAMPLE_RATE as usize,
            )
        };
        let midi_samples =
            make_samples_from_midi(sampler, DEFAULT_SAMPLE_RATE as usize, true, song.clone())
                .expect("midi play failed");
        Self {
            midi_samples,
            num_sample: 0,
            speed: 1.0,
            uses_speed,
            samples,
            song,
        }
    }
}

impl Iterator for MidiRenderer {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        if self.num_sample >= self.midi_samples.len() {
            self.num_sample = 0;
        }
        Some(self.midi_samples[self.num_sample] as f32)
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

impl SetSpeed<f32> for MidiRenderer {
    fn set_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
        let sampler = |frequency: f64| {
            wave::sampler(
                frequency / self.speed as f64,
                &self.samples,
                self.samples.len(),
                MIDDLE_C_FREQ as f64,
                DEFAULT_SAMPLE_RATE as usize,
            )
        };
        self.midi_samples = make_samples_from_midi(
            sampler,
            DEFAULT_SAMPLE_RATE as usize,
            false,
            self.song.clone(),
        )
        .expect("midi play failed");
    }
}
