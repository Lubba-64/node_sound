use crate::node_prelude::*;
use rand::Rng;
use rand::thread_rng;

#[derive(Clone, Debug)]
pub struct Noise {
    min: f32,
    max: f32,
}

impl Noise {
    #[inline]
    pub fn new(min: f32, max: f32) -> Self {
        let mut min_1 = min;
        let mut max_1 = max;
        if min_1 > max_1 {
            let other = min_1;
            min_1 = max_1;
            max_1 = other;
        }
        Self {
            min: min_1,
            max: max_1,
        }
    }
}

impl SoundNode for Noise {
    fn next(&mut self, _index: f32, _channel: u8) -> Option<f32> {
        if self.min == self.max {
            return Some(self.min);
        }
        Some(thread_rng().gen_range(self.min..self.max))
    }
}

pub fn noise_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Noise".to_string(),
        tooltip: r#"Random noise waveform."#.to_string(),
        inputs: BTreeMap::from([
            (
                "min".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "min".to_string(),
                    value: InputValueConfig::Float {
                        value: -1.0,
                        min: -1.0,
                        max: 1.0,
                    },
                },
            ),
            (
                "max".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "max".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: -1.0,
                        max: 1.0,
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
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(Noise::new(
                        props.get_float("min")?,
                        props.get_float("max")?,
                    ))),
                },
            )]))
        })),
    }
}
