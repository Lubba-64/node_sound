use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Wrapper<I: SoundNode> {
    source: I,
    last: [Option<f32>; 2],
}

impl<I: SoundNode> Wrapper<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self {
            source,
            last: [None; 2],
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Wrapper<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.last[channel as usize] = match (
            self.source.next(index, channel),
            self.last[channel as usize],
        ) {
            (Some(sample1), Some(sample2)) => {
                if sample1 + sample2 > 1.0 {
                    return Some(-1.0 + sample1 + sample2 - 1.0);
                }
                if sample1 + sample2 < -1.0 {
                    return Some(1.0 - sample1 + sample2 + 1.0);
                }
                Some(sample1 + sample2)
            }
            (None, Some(sample2)) => Some(sample2),
            (Some(sample1), None) => Some(sample1),
            _ => Some(0.0),
        };
        self.last[channel as usize]
    }
}

pub fn wrapper_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Wrapper".to_string(),
        tooltip: r#"Takes the last value and adds the current value to it.
wraps this value around to the other side if it exceeds -1.0 to 1.0."#
            .to_string(),
        inputs: BTreeMap::from([(
            "audio 1".to_string(),
            InputParameter {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "audio 1".to_string(),
                value: InputValueConfig::AudioSource {},
            },
        )]),
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
                    value: props.push_sound(Box::new(Wrapper::new(cloned))),
                },
            )]))
        })),
    }
}
