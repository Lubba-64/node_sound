use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_graph::DEFAULT_SAMPLE_RATE;
use crate::sound_map;
use crate::sounds::{samples_from_source, WavetableOscillator};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn wave_table_node() -> SoundNode {
    SoundNode {
        name: "Wave Table".to_string(),
        inputs: BTreeMap::from([
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "frequency".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "frequency".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: 0.0,
                        max: 4000.0,
                    },
                },
            ),
            (
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "duration".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 1.0,
                        max: 4000.0,
                    },
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

pub fn wave_table_logic(props: SoundNodeProps) -> SoundNodeResult {
    let samples = samples_from_source(
        sound_map::clone_sound_ref(props.get_source("audio 1")?)?,
        props.get_float("duration")? as usize,
    );

    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound(Box::new(
                WavetableOscillator::new(DEFAULT_SAMPLE_RATE, samples)
                    .set_frequency(props.get_float("frequency")?),
            )),
        },
    )]))
}
