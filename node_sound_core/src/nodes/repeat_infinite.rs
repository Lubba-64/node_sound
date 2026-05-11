use crate::{node_prelude::*, nodes::repeat::RepeatRefSource};

pub fn repeat_infinite_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Repeat Infinite".to_string(),
        tooltip: r#"Repeats a given waveform infinitely if it stops."#.to_string(),
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
            let cloned =
                RepeatRefSource::new(props.clone_sound(props.get_source("audio 1")?)?, None);
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(cloned)),
                },
            )]))
        })),
    }
}
