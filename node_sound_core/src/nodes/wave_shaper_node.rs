use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_graph::{DEFAULT_SAMPLE_RATE, MIDDLE_C_FREQ};
use crate::sound_map;
use crate::sounds::WavetableOscillator;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn wave_shaper_node() -> SoundNode {
    SoundNode {
        name: "Wave Shaper".to_string(),
        inputs: BTreeMap::from([(
            "graph".to_string(),
            InputParameter {
                data_type: DataType::Graph,
                kind: InputParamKind::ConstantOnly,
                name: "graph".to_string(),
                value: InputValueConfig::Graph {
                    value: vec![0.01; 1000],
                },
            },
        )]),
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}
pub fn wave_shaper_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound(Box::new(
                WavetableOscillator::new(
                    DEFAULT_SAMPLE_RATE,
                    props.get_graph("graph")?.unwrap_or(vec![0.01; 1000]),
                )
                .set_frequency(MIDDLE_C_FREQ),
            )),
        },
    )]))
}
