use crate::{
    constants::{DEFAULT_SAMPLE_RATE, MIDDLE_C_FREQ},
    sound_map::DawSource,
    sounds::wave_table::WaveTableOscillator,
};
use synthrs::{midi::MidiSong, synthesizer::make_samples_from_midi, wave};

#[derive(Clone)]
pub struct MidiRenderer<S: DawSource> {
    wavetable: WaveTableOscillator,
    song: MidiSong,
    source: S,
}

impl<S: DawSource> MidiRenderer<S> {
    #[inline]
    pub fn new(source: S, song: MidiSong, uses_speed: bool) -> Self {
        let sample_rate = DEFAULT_SAMPLE_RATE as f32;
        let mut osc = WaveTableOscillator::new_stereo(sample_rate, 1.0);
        osc.set_uses_speed(uses_speed);
        Self {
            wavetable: osc,
            song,
            source,
        }
    }

    fn render_midi_samples(&mut self, song: &MidiSong, sample_rate: f32, speed: f32) {
        self.source.note_speed(speed, sample_rate);
        let mut source_samples: Vec<f64> = Vec::with_capacity(sample_rate as usize);
        for i in 0..sample_rate as usize {
            let sample = self.source.next(i as f32, 0).unwrap_or(0.0);
            source_samples.push(sample.into());
        }
        let sampler = |frequency: f64| {
            wave::sampler(
                frequency * speed as f64,
                &source_samples,
                source_samples.len(),
                MIDDLE_C_FREQ as f64,
                sample_rate as usize,
            )
        };
        let midi_samples =
            make_samples_from_midi(sampler, sample_rate as usize, true, song.clone())
                .expect("midi play failed");
        let left: Vec<_> = midi_samples.iter().map(|&x| x as f32).collect();
        self.wavetable.note_speed(speed, sample_rate);
        self.wavetable
            .rebuild_table(sample_rate, left.clone(), left);
    }
}

impl<S: DawSource + Clone> DawSource for MidiRenderer<S> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.wavetable.next(index, channel)
    }

    fn note_speed(&mut self, speed: f32, rate: f32) {
        self.render_midi_samples(&self.song.clone(), rate, speed);
    }

    fn size_hint(&self) -> Option<f32> {
        self.wavetable.size_hint()
    }
}
