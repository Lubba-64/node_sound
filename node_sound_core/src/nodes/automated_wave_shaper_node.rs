use crate::constants::{DEFAULT_SAMPLE_RATE, WAVE_TABLE_SIZE};
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map;
use crate::sounds::AutomatedWavetableOscillator;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn automated_wave_shaper_node() -> SoundNode {
    SoundNode {
        name: "Automated Wave Shaper".to_string(),
        inputs: BTreeMap::from([
            (
                "graph".to_string(),
                InputParameter {
                    data_type: DataType::Graph,
                    kind: InputParamKind::ConstantOnly,
                    name: "graph".to_string(),
                    value: InputValueConfig::Graph {
                        value: vec![0.0; WAVE_TABLE_SIZE],
                    },
                },
            ),
            (
                "frequency".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "frequency".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
        ]),
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}

pub fn automated_wave_shaper_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound(Box::new(AutomatedWavetableOscillator::new(
                DEFAULT_SAMPLE_RATE,
                props
                    .get_graph("graph")?
                    .unwrap_or(vec![0.01; WAVE_TABLE_SIZE]),
                sound_map::clone_sound_ref(props.get_source("frequency")?)?,
            ))),
        },
    )]))
}
