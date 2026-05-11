use crate::node_prelude::*;
// AI generated. It works. I will eventually rewrite this from scratch.

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
    type Err = crate::error::NodeSoundError;
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
                return Err(anyhow!("Incorrect from_str value").into());
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct SingleFilterEq<I: SoundNode> {
    source: I,
    filter_type: FilterType,
    frequency: f32,
    q_factor: f32,
    gain: f32,
    sample_rate: f32,
    state: Vec<(f32, f32, f32, f32)>,
}

impl<I: SoundNode> SingleFilterEq<I> {
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

impl<I: SoundNode + Clone> SoundNode for SingleFilterEq<I> {
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

pub fn eq_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Eq".to_string(),
        tooltip: r#"Basic runtime EQ.
        Q factor controls how "selective" or "narrow" the filter is around the cutoff frequency.
        Low / High shelf and peak are the only settings that use gain. gain is in DB.
        it boosts / cuts the selected frequencies.
        Low pass filter removes frequencies above frequency.
        High pass filter removes frequencies below frequency.
        Band pass filter removes frequencies outside around frequency.
        Notch filter removes frequencies inside around frequency.
        Low shelf is an agressive low pass that cuts to 0 instead of moving smoothly.
        High shelf is an agressive high pass that cuts to 0 instead of moving smoothly.
        Peak EQ is very similar to Band pass, but width is controlled by q factor.
        "#
        .to_string(),
        inputs: BTreeMap::from([
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "filter type".to_string(),
                InputParameter {
                    data_type: DataType::Dropdown,
                    kind: InputParamKind::ConstantOnly,
                    name: "filter type".to_string(),
                    value: InputValueConfig::Dropdown {
                        value: FilterType::LowPass.to_string(),
                        values: FilterType::ALL.map(|filter| filter.to_string()).to_vec(),
                    },
                },
            ),
            (
                "frequency".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "frequency".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: 0.0,
                        max: MAX_FREQ,
                    },
                },
            ),
            (
                "q factor".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "q factor".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.7,
                        min: 0.7,
                        max: 10.0,
                    },
                },
            ),
            (
                "gain".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "gain".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: -12.0,
                        max: 12.0,
                    },
                },
            ),
        ]),
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
        op: Some(Arc::new(|mut props| {
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            let filter_type = FilterType::from_str(&props.get_dropdown("filter type")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(SingleFilterEq::new(
                        cloned,
                        props.sample_rate(),
                        2,
                        filter_type,
                        props.get_float("frequency")?,
                        props.get_float("q factor")?,
                        props.get_float("gain")?,
                    ))),
                },
            )]))
        })),
    }
}
