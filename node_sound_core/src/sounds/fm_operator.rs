use std::{f32::consts::PI, fmt::Debug};

use crate::sound_map::{DawSource, GenericSource, Oscillator};

#[derive(Clone, Debug)]
pub struct FmOperator<O: Oscillator> {
    oscillator: O,
    volume: f32,
    mix_out: f32,
    mod_out: f32,
    feedback: f32,
    panning: f32,
    envelope_volume: f32,
    velocity_sensitivity_mod_out: f32,
    velocity_sensitivity_feedback: f32,
    modulation_inputs: Vec<GenericSource>,
    key_velocity: f32,
    time_per_sample: f32,
    last_index: f32,
}

impl<O: Oscillator> FmOperator<O> {
    pub fn new(
        oscillator: O,
        volume: f32,
        mix_out: f32,
        mod_out: f32,
        feedback: f32,
        panning: f32,
        envelope_volume: f32,
        velocity_sensitivity_mod_out: f32,
        velocity_sensitivity_feedback: f32,
        time_per_sample: f32,
        key_velocity: f32,
        modulation1: GenericSource,
        modulation2: GenericSource,
    ) -> Self {
        Self {
            oscillator,
            volume,
            mix_out,
            mod_out,
            feedback,
            panning,
            envelope_volume,
            velocity_sensitivity_mod_out,
            velocity_sensitivity_feedback,
            modulation_inputs: vec![modulation1, modulation2],
            key_velocity,
            time_per_sample: 0.0,
            last_index: 0.0,
        }
    }

    fn velocity_factor(&self, sensitivity: f32, velocity: f32) -> f32 {
        sensitivity * velocity + (2.0 * PI - sensitivity)
    }

    fn process_sample(&mut self, index: f32, channel: u8) -> f32 {
        // Sum all modulation inputs
        let mut total_modulation_input = 0.0;
        if let Some(mod_value) = self.modulation_inputs[channel as usize].next(index, 0) {
            total_modulation_input += mod_value;
        }

        // Apply feedback to phase (like original code)
        let current_phase = self.oscillator.get_phase();
        let feedback_effect = self.feedback
            * self.velocity_factor(self.velocity_sensitivity_feedback, self.key_velocity)
            * self.oscillator.calculate_output();

        // Calculate new phase with modulation and feedback
        let frequency = self.oscillator.get_frequency();
        let new_phase = (current_phase
            + frequency * self.time_per_sample * 2.0 * PI
            + total_modulation_input
            + feedback_effect)
            % 2.0
            * PI;
        self.oscillator.set_phase(new_phase);

        // Generate output
        let sample = self.oscillator.calculate_output();
        let final_sample = sample * self.volume * self.envelope_volume;

        // Calculate mix out and mod out (like original code)
        let mix_out = final_sample * self.mix_out;

        let velocity_factor =
            self.velocity_factor(self.velocity_sensitivity_mod_out, self.key_velocity);
        // let mod_out = final_sample * self.mod_out * velocity_factor;

        mix_out
    }
}

impl<O: Oscillator + Clone> DawSource for FmOperator<O> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        // Handle time delta calculation similar to your FMSynth
        let time_delta = if index == 0.0 {
            1.0
        } else {
            if (index - self.last_index) < 0.0 {
                1.0
            } else {
                index - self.last_index
            }
        };
        self.last_index = index;

        // Adjust time per sample based on time delta
        self.time_per_sample = time_delta;

        let mix_out = self.process_sample(index, channel);

        // Apply panning based on channel
        let panned_output = match channel {
            0 => mix_out * (1.0 - self.panning), // Left channel
            1 => mix_out * self.panning,         // Right channel
            _ => mix_out,
        };

        Some(panned_output)
    }
}
