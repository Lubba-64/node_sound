use crate::sound_graph::types::{
    DataType, InputParameter, InputValueConfig, Output, SoundNode, ValueType,
};
use egui_node_graph::InputParamKind;
use std::collections::HashMap;

use super::{AsFiniteSource, TriangleWave};

pub fn traingle_node() -> SoundNode {
    SoundNode {
        name: "Triangle Wave".to_string(),
        inputs: HashMap::from([
            (
                "frequency".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "frequency".to_string(),
                    value: InputValueConfig::Float { value: 0.0 },
                },
            ),
            (
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
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
        operation: |x| {
            let freq = x.get("frequency").unwrap().clone().try_to_float().unwrap();
            let duration = x
                .get("duration")
                .unwrap()
                .clone()
                .try_to_duration()
                .unwrap();
            HashMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: TriangleWave::new(freq).as_generic(duration),
                },
            )])
        },
    }
}
