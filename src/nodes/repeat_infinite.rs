use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_queue;
use crate::sounds::AsGenericSource;
use egui_node_graph_2::InputParamKind;
use rodio::Source;
use std::collections::HashMap;

pub fn repeat_infinite_node() -> SoundNode {
    SoundNode {
        name: "Repeat Infinite".to_string(),
        inputs: HashMap::from([(
            "audio 1".to_string(),
            InputParameter {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "audio 1".to_string(),
                value: InputValueConfig::AudioSource {},
            },
        )]),
        outputs: HashMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
        operation: |props| {
            Ok(HashMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: sound_queue::push_sound(
                        sound_queue::clone_sound(props.get_source("audio 1")?)?
                            .repeat_infinite()
                            .as_generic(None),
                    ),
                },
            )]))
        },
    }
}
