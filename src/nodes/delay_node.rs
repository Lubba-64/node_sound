use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_queue;
use crate::sounds::AsGenericSource;
use egui_node_graph::InputParamKind;
use rodio::Source;
use std::collections::HashMap;

pub fn delay_node() -> SoundNode {
    SoundNode {
        name: "Delay".to_string(),
        inputs: HashMap::from([
            (
                "delay".to_string(),
                InputParameter {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
                },
            ),
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
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
            let duration = props
                .inputs
                .get("delay")
                .unwrap()
                .clone()
                .try_to_duration()
                .unwrap();

            let first = props
                .inputs
                .get("audio 1")
                .unwrap()
                .clone()
                .try_to_source()
                .unwrap();

            let idx = sound_queue::push_sound(
                sound_queue::clone_sound(first)
                    .delay(duration)
                    .as_generic(None),
            );

            HashMap::from([("out".to_string(), ValueType::AudioSource { value: idx })])
        },
    }
}
