use crate::sound_graph::types::{
    DataType, InputParameter, InputValueConfig, Output, SoundNode, ValueType,
};
use egui_node_graph::InputParamKind;
use std::collections::HashMap;

use super::SawToothWave;

pub fn sawtooth_node() -> SoundNode {
    SoundNode {
        name: "Sawtooth Wave".to_string(),
        inputs: HashMap::from([(
            "frequency".to_string(),
            InputParameter {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOrConstant,
                name: "frequency".to_string(),
                value: InputValueConfig::Float { value: 0.0 },
            },
        )]),
        outputs: HashMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
        operation: |hash, stack| {
            let freq = hash
                .get("frequency")
                .unwrap()
                .clone()
                .try_to_float()
                .unwrap();
            stack.push(Box::new(SawToothWave::new(freq)));
            HashMap::from([("out".to_string(), ValueType::AudioSource { value: 0 })])
        },
    }
}
