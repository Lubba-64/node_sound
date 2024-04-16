use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_queue;
use crate::sounds::{AsGenericSource, SquareWave};
use egui_node_graph_2::InputParamKind;
use std::collections::HashMap;
pub fn square_node() -> SoundNode {
    SoundNode {
        name: "Square Wave".to_string(),
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
        operation: |props| {
            let freq = props
                .inputs
                .get("frequency")
                .unwrap()
                .clone()
                .try_to_float()
                .unwrap();

            let idx = sound_queue::push_sound(SquareWave::new(freq).as_generic(None));

            HashMap::from([("out".to_string(), ValueType::AudioSource { value: idx })])
        },
    }
}
