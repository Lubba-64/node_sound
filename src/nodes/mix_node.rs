use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, SoundNode, ValueType,
};
use crate::sound_queue;
use crate::sounds::AsGenericSource;
use egui_node_graph::InputParamKind;
use rodio::Source;
use std::collections::HashMap;

pub fn mix_node() -> SoundNode {
    SoundNode {
        name: "Mix".to_string(),
        inputs: HashMap::from([
            (
                "audio source 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio source 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "audio source 2".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio source 2".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
        ]),
        outputs: HashMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
        operation: |props| {
            let first = props
                .inputs
                .get("audio source 1")
                .unwrap()
                .clone()
                .try_to_source()
                .unwrap();
            let second = props
                .inputs
                .get("audio source 2")
                .unwrap()
                .clone()
                .try_to_source()
                .unwrap();

            let new_sound = sound_queue::push_sound(
                sound_queue::get_sound(first)
                    .mix(sound_queue::get_sound(second))
                    .as_generic(None, None),
            );

            HashMap::from([(
                "out".to_string(),
                ValueType::AudioSource { value: new_sound },
            )])
        },
    }
}
