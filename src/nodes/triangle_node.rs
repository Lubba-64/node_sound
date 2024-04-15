use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, SoundNode, ValueType,
};
use crate::sound_queue;
use crate::sounds::{AsGenericSource, TriangleWave};
use egui_node_graph::InputParamKind;
use std::collections::HashMap;

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

            let idx = sound_queue::push_sound(TriangleWave::new(freq).as_generic(
                Some(duration),
                Some(*props.output_connection_counts.get("out").unwrap()),
            ));

            HashMap::from([("out".to_string(), ValueType::AudioSource { value: idx })])
        },
    }
}
