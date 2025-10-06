use std::{f32::consts::PI, str::FromStr};

use crate::sound_map::DawSource;
// yes this is AI, I would not understand how to do this myself but if it works its getting added!
// this stuff is just for fun.

#[derive(Clone, Copy, Debug)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    Notch,
    LowShelf,
    HighShelf,
    Peak,
}

impl FilterType {
    pub const ALL: [FilterType; 7] = [
        FilterType::LowPass,
        FilterType::HighPass,
        FilterType::BandPass,
        FilterType::Notch,
        FilterType::LowShelf,
        FilterType::HighShelf,
        FilterType::Peak,
    ];
}

impl ToString for FilterType {
    fn to_string(&self) -> String {
        match self {
            FilterType::LowPass => "LowPass".to_string(),
            FilterType::HighPass => "HighPass".to_string(),
            FilterType::BandPass => "BandPass".to_string(),
            FilterType::Notch => "Notch".to_string(),
            FilterType::LowShelf => "LowShelf".to_string(),
            FilterType::HighShelf => "HighShelf".to_string(),
            FilterType::Peak => "Peak".to_string(),
        }
    }
}

impl FromStr for FilterType {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "LowPass" => FilterType::LowPass,
            "HighPass" => FilterType::HighPass,
            "BandPass" => FilterType::BandPass,
            "Notch" => FilterType::Notch,
            "LowShelf" => FilterType::LowShelf,
            "HighShelf" => FilterType::HighShelf,
            "Peak" => FilterType::Peak,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Incorrect from_str value",
                ));
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct SingleFilterEq<I: DawSource> {
    source: I,
    filter_type: FilterType,
    frequency: f32,
    q_factor: f32,
    gain: f32,
    sample_rate: f32,
    state: Vec<(f32, f32, f32, f32)>,
}

impl<I: DawSource> SingleFilterEq<I> {
    pub fn new(
        source: I,
        sample_rate: f32,
        channels: usize,
        filter_type: FilterType,
        frequency: f32,
        q_factor: f32,
        gain: f32,
    ) -> Self {
        Self {
            source,
            filter_type,
            frequency,
            q_factor,
            gain,
            sample_rate,
            state: vec![(0.0, 0.0, 0.0, 0.0); channels],
        }
    }

    fn calculate_coefficients(&self) -> (f32, f32, f32, f32, f32, f32) {
        let omega = 2.0 * PI * self.frequency / self.sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * self.q_factor);

        match self.filter_type {
            FilterType::LowPass => {
                let b0 = (1.0 - cos_omega) / 2.0;
                let b1 = 1.0 - cos_omega;
                let b2 = (1.0 - cos_omega) / 2.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::HighPass => {
                let b0 = (1.0 + cos_omega) / 2.0;
                let b1 = -(1.0 + cos_omega);
                let b2 = (1.0 + cos_omega) / 2.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::BandPass => {
                let b0 = alpha;
                let b1 = 0.0;
                let b2 = -alpha;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::Notch => {
                let b0 = 1.0;
                let b1 = -2.0 * cos_omega;
                let b2 = 1.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::LowShelf => {
                let a = 10.0f32.powf(self.gain / 40.0);
                let beta = (a + a.sqrt()) * sin_omega;
                let b0 = a * ((a + 1.0) - (a - 1.0) * cos_omega + beta);
                let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_omega);
                let b2 = a * ((a + 1.0) - (a - 1.0) * cos_omega - beta);
                let a0 = (a + 1.0) + (a - 1.0) * cos_omega + beta;
                let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_omega);
                let a2 = (a + 1.0) + (a - 1.0) * cos_omega - beta;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::HighShelf => {
                let a = 10.0f32.powf(self.gain / 40.0);
                let beta = (a + a.sqrt()) * sin_omega;
                let b0 = a * ((a + 1.0) + (a - 1.0) * cos_omega + beta);
                let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_omega);
                let b2 = a * ((a + 1.0) + (a - 1.0) * cos_omega - beta);
                let a0 = (a + 1.0) - (a - 1.0) * cos_omega + beta;
                let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_omega);
                let a2 = (a + 1.0) - (a - 1.0) * cos_omega - beta;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::Peak => {
                let a = 10.0f32.powf(self.gain / 40.0);
                let alpha = sin_omega / (2.0 * self.q_factor);
                let b0 = 1.0 + alpha * a;
                let b1 = -2.0 * cos_omega;
                let b2 = 1.0 - alpha * a;
                let a0 = 1.0 + alpha / a;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha / a;
                (b0, b1, b2, a0, a1, a2)
            }
        }
    }

    fn apply_filter(&mut self, sample: f32, channel: usize) -> f32 {
        let (b0, b1, b2, a0, a1, a2) = self.calculate_coefficients();
        let state = &mut self.state[channel];

        // Biquad filter difference equation:
        // y[n] = (b0*x[n] + b1*x[n-1] + b2*x[n-2] - a1*y[n-1] - a2*y[n-2]) / a0
        let output = (b0 * sample + b1 * state.0 + b2 * state.1 - a1 * state.2 - a2 * state.3) / a0;

        state.1 = state.0;
        state.0 = sample;
        state.3 = state.2;
        state.2 = output;

        output
    }
}

impl<I: DawSource + Clone> DawSource for SingleFilterEq<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let channel_idx = channel as usize;
        if channel_idx >= self.state.len() {
            self.state.resize(channel_idx + 1, (0.0, 0.0, 0.0, 0.0));
        }
        if let Some(sample) = self.source.next(index, channel) {
            Some(self.apply_filter(sample, channel_idx))
        } else {
            None
        }
    }
}
