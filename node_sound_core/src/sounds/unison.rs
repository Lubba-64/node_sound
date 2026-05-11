use crate::{
    node::{GenericSoundNode, SoundNode},
    sounds::{skip::Skip, speed::Speed},
};

#[derive(Clone, Debug)]
pub struct UnisonVoice {
    source: MixVec,
}

#[derive(Clone, Debug)]
pub struct MixVec {
    vec: Vec<GenericSoundNode>,
}

impl MixVec {
    fn new(vec: Vec<GenericSoundNode>) -> Self {
        Self { vec }
    }
}

impl SoundNode for MixVec {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let mut samples = vec![0.0; self.vec.len()];
        for i in 0..self.vec.len() {
            samples[i] = self.vec[i].next(index, channel).unwrap_or_default();
        }
        Some(samples.iter().sum::<f32>() / (self.vec.len() as f32).sqrt())
    }
}

impl UnisonVoice {
    pub fn new<S: SoundNode + 'static + Clone>(
        source: S,
        mut phase_sep: f32,
        voices: u8,
        sample_rate: f32,
        note_speed: f32,
        base_frequency: f32,
        detune_amount: f32,
    ) -> Self {
        phase_sep = phase_sep.abs().clamp(0.0, 100.0) / 100.0;

        // Convert phase separation to time offset (seconds)
        let time_offset = phase_sep * (1.0 / base_frequency);

        let mut mix_vec = Vec::new();

        for i in 0..voices {
            // Symmetric detune distribution around center
            let detune_factor = if voices > 1 {
                let position = (i as f32) / ((voices - 1) as f32) - 0.5;
                1.0 + detune_amount * position * 0.01
            } else {
                1.0
            };

            // Apply detune to the note speed
            let detuned_speed = note_speed * detune_factor;

            // Create the voice source with phase offset
            let voice_source = if i == 0 {
                // First voice - no phase offset, just potential detune
                if (detune_factor - 1.0).abs() > 0.0001 {
                    // Only wrap in Speed if detune is actually applied
                    GenericSoundNode::new(Box::new(Speed::new(source.clone(), detuned_speed)))
                } else {
                    GenericSoundNode::new(Box::new(source.clone()))
                }
            } else {
                // Other voices with phase offset and detune
                let phase_offset = time_offset * i as f32;

                GenericSoundNode::new(Box::new(Speed::new(
                    Skip::new(phase_offset, source.clone(), true, sample_rate, note_speed),
                    detuned_speed,
                )))
            };

            mix_vec.push(voice_source);
        }

        Self {
            source: MixVec::new(mix_vec),
        }
    }
}

impl SoundNode for UnisonVoice {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel)
    }
}
