use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_queue;
use crate::sounds::{AsGenericSource, SawToothWave};
use egui_node_graph::InputParamKind;
use std::collections::HashMap;

pub fn sawtooth_node() -> SoundNode {
    SoundNode {
        name: "Sawtooth Wave".to_string(),
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
        operation: |props| {
            let freq = props
                .inputs
                .get("frequency")
                .unwrap()
                .clone()
                .try_to_float()
                .unwrap();
            let duration = props
                .inputs
                .get("duration")
                .unwrap()
                .clone()
                .try_to_duration()
                .unwrap();

            let idx = sound_queue::push_sound(SawToothWave::new(freq).as_generic(Some(duration)));

            HashMap::from([("out".to_string(), ValueType::AudioSource { value: idx })])
        },
    }
}
