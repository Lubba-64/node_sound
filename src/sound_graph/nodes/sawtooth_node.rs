use crate::sound_graph::types::{
    DataType, InputParameter, InputValueConfig, Output, SoundNode, ValueType,
};
use egui_node_graph::InputParamKind;
use std::collections::HashMap;

use super::{AsFiniteSource, SawToothWave};

pub fn sawtooth_node() -> SoundNode {
    SoundNode {
        name: "sawtooth".to_string(),
        inputs: HashMap::from([
            (
                "frequency".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConstantOnly,
                    name: "frequency".to_string(),
                    value: InputValueConfig::Float { value: 0.0 },
                },
            ),
            (
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConstantOnly,
                    name: "duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
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
        operation: |x, sources| {
            let freq = x.get("frequency").unwrap().clone().try_to_float().unwrap();
            let duration = x
                .get("duration")
                .unwrap()
                .clone()
                .try_to_duration()
                .unwrap();
            sources.push(SawToothWave::new(freq).as_finite(duration));
            HashMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: sources.len() - 1,
                },
            )])
        },
    }
}
