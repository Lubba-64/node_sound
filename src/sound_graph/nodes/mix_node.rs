use crate::sound_graph::types::{
    DataType, InputParameter, InputValueConfig, Output, SoundNode, ValueType,
};
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
        operation: |hash, stack| {
            let first = hash
                .get("audio source 1")
                .unwrap()
                .clone()
                .try_to_source()
                .unwrap();
            let second = hash
                .get("audio source 2")
                .unwrap()
                .clone()
                .try_to_source()
                .unwrap();
            let x = stack.remove(first);
            let y = stack.remove(second);
            stack.push(Box::new(x.mix(y)));
            HashMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: stack.len() - 1,
                },
            )])
        },
    }
}
