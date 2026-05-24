use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct RawMod<I: SoundNode> {
    source: I,
    mod_by: f32,
}

impl<I: SoundNode> RawMod<I> {
    #[inline]
    pub fn new(source: I, mod_by: f32) -> Self {
        Self { source, mod_by }
    }
}

impl<I: SoundNode + Clone> SoundNode for RawMod<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match self.source.next(index, channel) {
            Some(sample) => Some(sample % self.mod_by),
            None => None,
        }
    }
}

pub fn mod_raw_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Mod Raw".to_string(),
        tooltip: r#"Takes the remainder of the wave and this value."#.to_string(),
        inputs: BTreeMap::from([
            (
                "mod".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "mod".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: -1.0,
                        max: 1.0,
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
                    value: props.push_sound(Box::new(RawMod::new(cloned, props.get_float("mod")?))),
                },
            )]))
        })),
    }
}
