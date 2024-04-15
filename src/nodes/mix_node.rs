use crate::sound_graph::types::{
    DataType, InputParameter, InputValueConfig, Output, SoundNode, ValueType,
};
use egui_node_graph::InputParamKind;
use rodio::Source;
use std::collections::HashMap;

use super::AsFiniteSource;

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
        operation: |x| {
            let first = x
                .get("audio source 1")
                .unwrap()
                .clone()
                .try_to_source()
                .unwrap();
            let second = x
                .get("audio source 2")
                .unwrap()
                .clone()
                .try_to_source()
                .unwrap();
            let duration = first.total_duration().unwrap();
            HashMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: first.mix(second).as_finite(duration),
                },
            )])
        },
    }
}
