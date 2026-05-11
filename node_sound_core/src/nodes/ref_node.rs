use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

pub fn ref_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Ref".to_string(),
        tooltip: r#"Does nothing to the sound itself,
Copies the result of a sound and caches it for each sample the graph produces (this is good for performance)."#
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
        op: Some(Arc::new(|mut props|{
            let arc_cloned = props
                .state
                .runtime_state
                .queue
                .arc_clone_sound(props.get_source("audio 1")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(arc_cloned)),
                },
            )]))
        }))
    }
}
