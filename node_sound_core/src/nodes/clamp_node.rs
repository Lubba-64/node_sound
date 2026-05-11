use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Clamp<I: SoundNode> {
    source: I,
    min: f32,
    max: f32,
}

impl<I: SoundNode> Clamp<I> {
    #[inline]
    pub fn new(source: I, mut min: f32, mut max: f32) -> Self {
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }
        Self {
            source,
            max: max,
            min: min,
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Clamp<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        return self
            .source
            .next(index, channel)
            .map(|val| val.clamp(self.min, self.max));
    }
}

pub fn clamp_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Clamp".to_string(),
        tooltip: r#"Clamps the given waveform between a min and max value,
making sure no values go above or below the given min or max."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "min".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "min".to_string(),
                    value: InputValueConfig::Float {
                        value: -1.0,
                        max: 1.0,
                        min: -1.0,
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
                        max: 1.0,
                        min: -1.0,
                    },
                },
            ),
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
                    value: InputValueConfig::AudioSource {},
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
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(Clamp::new(
                        cloned,
                        props.get_float("min")?,
                        props.get_float("max")?,
                    ))),
                },
            )]))
        })),
    }
}
