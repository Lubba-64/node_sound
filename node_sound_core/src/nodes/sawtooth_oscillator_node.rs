use crate::constants::MAX_FREQ;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::sawtooth::SawtoothWave;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn sawtooth_oscillator_node() -> SoundNode {
    SoundNode {
        name: "Sawtooth Oscillator".to_string(),
        tooltip: r#"Sawtooth Oscillator."#.to_string(),
        inputs: BTreeMap::from([(
            "frequency".to_string(),
            InputParameter {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOrConstant,
                name: "frequency".to_string(),
                value: InputValueConfig::Float {
                    value: 0.0,
                    min: 0.0,
                    max: MAX_FREQ,
                },
            },
        )]),
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::Oscillator,
                name: "out".to_string(),
            },
        )]),
    }
}

pub fn sawtooth_oscillator_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::Oscillator {
            value: props.push_osc(Box::new(SawtoothWave::new(
                props.get_float("frequency")?,
                false,
                props.sample_rate(),
                props.note_speed(),
            ))),
        },
    )]))
}
