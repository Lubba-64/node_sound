use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct ClampToNote<I: SoundNode> {
    source: I,
}

impl<I: SoundNode> ClampToNote<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self { source }
    }
}

impl<I: SoundNode + Clone> SoundNode for ClampToNote<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        let val = self.source.next(index, channel).unwrap_or_default();
        let mut least_idx = 0;
        let mut least = f32::MAX;
        for (idx, pitch) in Pitch::ALL_FREQ.iter().enumerate() {
            let pitch_diff = (pitch - val).abs();
            if pitch_diff < least {
                least_idx = idx;
                least = pitch_diff;
            }
        }
        Some(Pitch::ALL[least_idx].match_freq())
    }
}

pub fn clamp_to_note_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Clamp To Note".to_string(),
        tooltip: r#"Clamps the incoming value to the nearest note value. Should only be used after translate wave."#
            .to_string(),
        inputs: BTreeMap::from([(
            "audio 1".to_string(),
            Input {
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
        op: Some(Arc::new(|mut props|{
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(ClampToNote::new(cloned))),
                },
            )]))
        }))
    }
}
