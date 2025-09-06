use crate::{
    constants::{DEFAULT_SAMPLE_RATE, MIDDLE_C_FREQ},
    sound_map::DawSource,
};
use synthrs::{midi::MidiSong, synthesizer::make_samples_from_midi, wave};

#[derive(Clone)]
pub struct MidiRenderer {
    midi_samples: Vec<f32>,
    speed: f32,
    uses_speed: bool,
    source_samples: Vec<f32>,
    song: MidiSong,
    sample_rate: f32,
}

impl MidiRenderer {
    #[inline]
    pub fn new<S: DawSource>(source: &mut S, song: MidiSong, uses_speed: bool) -> Self {
        let sample_rate = DEFAULT_SAMPLE_RATE as f32;
        let num_samples = sample_rate as usize;
        let mut source_samples = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let sample = source.next(i as f32, 0).unwrap_or(0.0);
            source_samples.push(sample);
        }
        let midi_samples = Self::render_midi_samples(&source_samples, &song, sample_rate, 1.0);
        Self {
            midi_samples,
            speed: 1.0,
            uses_speed,
            source_samples,
            song,
            sample_rate,
        }
    }

    fn render_midi_samples(
        source_samples: &[f32],
        song: &MidiSong,
        sample_rate: f32,
        speed: f32,
    ) -> Vec<f32> {
        let sampler = |frequency: f64| {
            wave::sampler(
                frequency / speed as f64,
                &source_samples.iter().map(|&x| x as f64).collect::<Vec<_>>(),
                source_samples.len(),
                MIDDLE_C_FREQ as f64,
                sample_rate as usize,
            )
        };

        make_samples_from_midi(sampler, sample_rate as usize, true, song.clone())
            .expect("midi play failed")
            .into_iter()
            .map(|x| x as f32)
            .collect()
    }

    pub fn get_duration(&self) -> f32 {
        self.midi_samples.len() as f32 / self.sample_rate
    }
}

impl DawSource for MidiRenderer {
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
        let len = self.midi_samples.len() as f32;
        if len == 0.0 {
            return Some(0.0);
        }
        let position = (index * self.speed) % len;
        let truncated = position as usize;
        if truncated >= self.midi_samples.len() {
            return Some(0.0);
        }
        let next = (truncated + 1) % self.midi_samples.len();
        let weight = position - truncated as f32;
        let sample =
            self.midi_samples[truncated] * (1.0 - weight) + self.midi_samples[next] * weight;

        Some(sample)
    }

    fn note_speed(&mut self, speed: f32) {
        if !self.uses_speed {
            return;
        }
        self.speed = speed;
        self.midi_samples = Self::render_midi_samples(
            &self.source_samples,
            &self.song,
            self.sample_rate,
            self.speed,
        );
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
        self.midi_samples = Self::render_midi_samples(
            &self.source_samples,
            &self.song,
            self.sample_rate,
            self.speed,
        );
    }
}
