use std::f32::consts::PI;

use crate::sound_map::{DawSource, Oscillator};

#[derive(Clone, Debug)]
pub struct FMSynth<C, M>
where
    C: Oscillator,
    M: Oscillator,
{
    carrier: C,
    modulator: M,
    modulation_index: f32,
    sample_rate: f32,
    last_index: f32,
}

impl<C, M> FMSynth<C, M>
where
    C: Oscillator,
    M: Oscillator,
{
    pub fn new(carrier: C, modulator: M, modulation_index: f32, sample_rate: f32) -> Self {
        Self {
            carrier,
            modulator,
            modulation_index,
            sample_rate,
            last_index: 0.0,
        }
    }
}

impl<C, M> DawSource for FMSynth<C, M>
where
    C: Oscillator + Clone,
    M: Oscillator + Clone,
{
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
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
        let modulator_phase = self.modulator.get_phase();
        let modulator_output = self.modulator.calculate_output();

        let carrier_phase = self.carrier.get_phase();

        // Apply FM modulation - phase is in [0, 2π] range
        let carrier_freq = self.carrier.get_frequency() * time_delta;
        let base_increment = (carrier_freq * 2.0 * PI) / self.sample_rate;
        let modulated_increment = base_increment + (self.modulation_index * modulator_output);

        let new_carrier_phase = (carrier_phase + modulated_increment) % (2.0 * PI);

        // Advance modulator phase normally
        let modulator_freq = self.modulator.get_frequency() * time_delta;
        let modulator_increment = (modulator_freq * 2.0 * PI) / self.sample_rate;
        let new_modulator_phase = (modulator_phase + modulator_increment) % (2.0 * PI);

        // Update phases
        self.carrier.set_phase(new_carrier_phase);
        self.modulator.set_phase(new_modulator_phase);

        // Use carrier's own calculation
        Some(self.carrier.calculate_output())
    }
}
