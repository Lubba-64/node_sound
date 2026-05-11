use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct AutomatedTranslateWave<
    I1: SoundNode,
    I2: SoundNode,
    I3: SoundNode,
    I4: SoundNode,
    I5: SoundNode,
> {
    source: I1,
    start_min: I2,
    start_max: I3,
    end_min: I4,
    end_max: I5,
}

impl<I1: SoundNode, I2: SoundNode, I3: SoundNode, I4: SoundNode, I5: SoundNode>
    AutomatedTranslateWave<I1, I2, I3, I4, I5>
{
    #[inline]
    pub fn new(source: I1, start_min: I2, start_max: I3, end_min: I4, end_max: I5) -> Self {
        Self {
            source,
            start_max,
            start_min,
            end_max,
            end_min,
        }
    }
}

impl<
    I1: SoundNode + Clone,
    I2: SoundNode + Clone,
    I3: SoundNode + Clone,
    I4: SoundNode + Clone,
    I5: SoundNode + Clone,
> SoundNode for AutomatedTranslateWave<I1, I2, I3, I4, I5>
{
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match (
            self.source.next(index, channel),
            self.start_min.next(index, channel),
            self.start_max.next(index, channel),
            self.end_min.next(index, channel),
            self.end_max.next(index, channel),
        ) {
            (
                Some(p),
                Some(mut start_min),
                Some(mut start_max),
                Some(mut end_min),
                Some(mut end_max),
            ) => {
                if start_min > start_max {
                    std::mem::swap(&mut start_min, &mut start_max);
                }
                if end_min > end_max {
                    std::mem::swap(&mut end_min, &mut end_max);
                }
                Some(
                    end_min
                        + ((end_max - end_min) / (start_max - start_min))
                            * (p.clamp(start_min, start_max) - start_min),
                )
            }
            _ => None,
        }
    }
}

pub fn automated_translate_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Translate Wave".to_string(),
        tooltip: r#"Automated version of the Translate node.
All parameters from the previous node are automated."#
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
                "start_max".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "start_max".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "start_min".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "start_min".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "end_max".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "end_max".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "end_min".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "end_min".to_string(),
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
            let cloned1 = props.clone_sound(props.get_source("start_min")?)?;
            let cloned2 = props.clone_sound(props.get_source("start_max")?)?;
            let cloned3 = props.clone_sound(props.get_source("end_min")?)?;
            let cloned4 = props.clone_sound(props.get_source("end_max")?)?;
            let cloned5 = props.clone_sound(props.get_source("audio 1")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(AutomatedTranslateWave::new(
                        cloned1, cloned2, cloned3, cloned4, cloned5,
                    ))),
                },
            )]))
        })),
    }
}
