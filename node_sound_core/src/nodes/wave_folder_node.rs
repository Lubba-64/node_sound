use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Wavefolder<I: SoundNode> {
    source: I,
    gain: f32,
    offset: f32,
    folds: u8,
    last_sample: f32,
}

impl<I: SoundNode> Wavefolder<I> {
    #[inline]
    pub fn new(source: I, gain: f32, offset: f32, folds: u8) -> Self {
        Self {
            source,
            gain,
            offset,
            folds,
            last_sample: 0.0,
        }
    }

    fn fold_wave(&mut self, mut sample: f32) -> f32 {
        sample = sample * self.gain + self.offset;

        for _ in 0..self.folds {
            sample += self.last_sample;
            sample = if sample > 1.0 {
                2.0 - sample
            } else if sample < -1.0 {
                -2.0 - sample
            } else {
                sample
            };
        }
        self.last_sample = sample;
        sample.clamp(-1.0, 1.0)
    }
}

impl<I: SoundNode + Clone> SoundNode for Wavefolder<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source
            .next(index, channel)
            .map(|sample| self.fold_wave(sample))
    }
}

pub fn wave_folder_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Wave Folder".to_string(),
        tooltip: r#"Distortion effect for folding a wave into itself repeatedly."#.to_string(),
        inputs: BTreeMap::from([
            (
                "gain".to_string(),
                Input {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "gain".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: 8.0,
                    },
                },
            ),
            (
                "offset".to_string(),
                Input {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "offset".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: -2.0,
                        max: 2.0,
                    },
                },
            ),
            (
                "folds".to_string(),
                Input {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "folds".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 1.0,
                        max: 8.0,
                    },
                },
            ),
            (
                "audio 1".to_string(),
                Input {
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
                    value: props.push_sound(Box::new(Wavefolder::new(
                        cloned,
                        props.get_float("gain")?,
                        props.get_float("offset")?,
                        props.get_float("folds")? as u8,
                    ))),
                },
            )]))
        })),
    }
}
