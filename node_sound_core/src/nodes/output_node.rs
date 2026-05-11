use crate::node_prelude::*;

pub fn output_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Output".to_string(),
        tooltip: r#"Finalized output audio to the DAW."#.to_string(),
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
        op: Some(Arc::new(|props| {
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.get_source("audio 1")?,
                },
            )]))
        })),
    }
}
