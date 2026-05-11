use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct BitCrusher<I: SoundNode> {
    source: I,
    step_size: f32,
}

impl<I: SoundNode> BitCrusher<I> {
    #[inline]
    pub fn new(source: I, bits: u32) -> Self {
        let bits = bits.clamp(1, 16);
        let step_size = 1.0 / bits as f32;
        Self { source, step_size }
    }
}

impl<I: SoundNode + Clone> SoundNode for BitCrusher<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source
            .next(index, channel)
            .map(|sample| ((sample / self.step_size).rem_euclid(self.step_size)).clamp(-1.0, 1.0))
    }
}

pub fn bit_crush_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Bit Crusher".to_string(),
        tooltip: r#"Bit chrushes the given waveform."#.to_string(),
        inputs: BTreeMap::from([
            (
                "reduction".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "reduction".to_string(),
                    value: InputValueConfig::Float {
                        value: 4.0,
                        max: 128.0,
                        min: 1.0,
                    },
                },
            ),
            (
                "audio".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio".to_string(),
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
            let cloned = props.clone_sound(props.get_source("audio")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(BitCrusher::new(
                        cloned,
                        props.get_float("reduction")? as u32,
                    ))),
                },
            )]))
        })),
    }
}
