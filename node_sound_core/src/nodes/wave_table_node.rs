use crate::constants::{DEFAULT_SAMPLE_RATE, MAX_FREQ, MIDDLE_C_FREQ};
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::wave_table::WavetableOscillator;
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
                        max: MAX_FREQ,
                    },
                },
            ),
            (
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
                },
            ),
            (
                "note independant".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "note independant".to_string(),
                    value: InputValueConfig::Bool { value: false },
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

pub fn wave_table_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let mut audio = WavetableOscillator::from_source(
        &mut props.clone_sound(props.get_source("audio 1")?)?,
        DEFAULT_SAMPLE_RATE,
        props.get_duration("duration")?.as_secs_f32(),
        MIDDLE_C_FREQ,
        props.get_bool("note independant")?,
    );
    audio.set_frequency(props.get_float("frequency")?);
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(audio)),
        },
    )]))
}
