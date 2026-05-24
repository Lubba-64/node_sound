use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct Grain<I: SoundNode, S: SoundNode, L: SoundNode> {
    current_source: I,
    ind_min: f32,
    start: S,
    len: L,
    sample_rate: f32,
    current_len: Option<f32>,
}

impl<I: SoundNode + Clone, S: SoundNode + Clone, L: SoundNode + Clone> Grain<I, S, L> {
    #[inline]
    pub fn new(source: I, start: S, len: L, sample_rate: f32) -> Self {
        Self {
            current_source: source,
            ind_min: 0.0,
            start,
            len,
            sample_rate,
            current_len: None,
        }
    }
}

impl<I: SoundNode + Clone, S: SoundNode + Clone, L: SoundNode + Clone> SoundNode
    for Grain<I, S, L>
{
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        let start = self.start.next(index, channel)? * self.sample_rate;
        if self.current_len.is_none() {
            self.current_len = Some(self.len.next(index, channel)? * self.sample_rate);
        }
        index -= self.ind_min;
        if index > self.current_len.unwrap_or_default() {
            self.current_len = Some(self.len.next(index, channel)? * self.sample_rate);
            self.ind_min += index;
        }
        index += start;
        self.current_source.next(index, channel)
    }
}

pub fn grain_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Grain".to_string(),
        tooltip: r#"Repeating of a given grain where the length and start of the grain can be automated."#.to_string(),
        inputs: BTreeMap::from([
            (
                "audio 1".to_string(),
                Input {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "start".to_string(),
                Input {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "start".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "len".to_string(),
                Input {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "len".to_string(),
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
        op: Some(Arc::new(|mut props|{
            let cloned = Grain::new(
                props.clone_sound(props.get_source("audio 1")?)?,
                props.clone_sound(props.get_source("start")?)?,
                props.clone_sound(props.get_source("len")?)?,
                props.sample_rate(),
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(cloned)),
                },
            )]))
        }))
    }
}
