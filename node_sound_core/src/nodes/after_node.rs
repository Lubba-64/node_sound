use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct After<T: SoundNode, U: SoundNode> {
    first: T,
    second: U,
    current: AfterState,
}

#[derive(Clone, Debug)]
enum AfterState {
    PlayingFirst,
    PlayingSecond,
    Finished,
}

impl<T: SoundNode, U: SoundNode> After<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Self {
            first,
            second,
            current: AfterState::PlayingFirst,
        }
    }
}

impl<T: SoundNode + Clone, U: SoundNode + Clone> SoundNode for After<T, U> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match self.current {
            AfterState::PlayingFirst => {
                if let Some(sample) = self.first.next(index, channel) {
                    Some(sample)
                } else {
                    self.current = AfterState::PlayingSecond;
                    self.second.next(index, channel)
                }
            }
            AfterState::PlayingSecond => {
                if let Some(sample) = self.second.next(index, channel) {
                    Some(sample)
                } else {
                    self.current = AfterState::Finished;
                    None
                }
            }
            AfterState::Finished => None,
        }
    }
}

pub fn after_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "After".to_string(),
        tooltip: r#"Plays audio 2 after audio 1 has finished playing."#.to_string(),
        inputs: vec![
            Input {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "audio 1".to_string(),
                value: InputValueConfig::AudioSource {},
            },
            Input {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "audio 2".to_string(),
                value: InputValueConfig::AudioSource {},
            },
        ],
        outputs: get_default_outputs(),
        op: Some(Arc::new(|mut props| {
            let cloned1 = props.clone_sound(props.get_source("audio 1")?)?;
            let cloned2 = props.clone_sound(props.get_source("audio 2")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(After::new(cloned1, cloned2))),
                },
            )]))
        })),
    }
}
