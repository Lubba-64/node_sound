use egui_node_graph::InputParamKind;
use std::collections::HashMap;
use std::string::String;

use crate::sound_graph::types::{
    DataType, InputParameter, InputValueConfig, Output, SoundNode, ValueType,
};
/*
use super::SawToothWave;

pub fn sawtooth_node() -> SoundNode {
    SoundNode {
        name: "sawtooth".to_string(),
        inputs: vec![InputParameter {
            data_type: DataType::Float,
            kind: InputParamKind::ConstantOnly,
            name: "frequency".to_string(),
            value: InputValueConfig::Float { value: 0.0 },
        }],
        outputs: vec![Output {
            data_type: DataType::AudioSource,
            name: "out".to_string(),
        }],
        operation: |x, sources| {
            let saw = SawToothWave::new(x.get("frequency").unwrap().try_to_float().unwrap());
            sources.append(Box::new(saw));
            DataType::AudioSource { value }
        },
    }
}
*/
