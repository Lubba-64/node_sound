use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct SineWave {
    frequency: f32,
    speed: f32,
    sample_rate: f32,
}

impl SineWave {
    #[inline]
    pub fn new(frequency: f32, uses_speed: bool, sample_rate: f32, speed: f32) -> Self {
        Self {
            frequency,
            speed: if uses_speed { speed } else { 1.0 },
            sample_rate,
        }
    }
}

impl SoundNode for SineWave {
    fn next(&mut self, index: f32, _channel: u8) -> Option<f32> {
        let phase_increment = (2.0 * PI) * self.frequency / self.sample_rate / self.speed;
        let phase = (phase_increment * index) % (2.0 * PI);
        Some(phase.sin())
    }
}

pub fn sine_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Sine Wave".to_string(),
        tooltip: r#"Sine waveform generator."#.to_string(),
        inputs: BTreeMap::from([
            (
                "frequency".to_string(),
                Input {
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
                "note independant".to_string(),
                Input {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "note independant".to_string(),
                    value: InputValueConfig::Bool { value: false },
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
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(SineWave::new(
                        props.get_float("frequency")?,
                        props.get_bool("note independant")?,
                        props.sample_rate(),
                        props.note_speed(),
                    ))),
                },
            )]))
        })),
    }
}
