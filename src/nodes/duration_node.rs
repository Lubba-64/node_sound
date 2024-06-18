use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map::{self, RefSource};
use egui_node_graph_2::InputParamKind;
use rodio::source::TakeDuration;
use rodio::Source;
use std::collections::HashMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn duration_node() -> SoundNode {
    SoundNode {
        name: "Take Duration".to_string(),
        inputs: HashMap::from([
            (
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
                },
            ),
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
                    value: InputValueConfig::AudioSource {},
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
    }
}

pub fn duration_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(HashMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound::<TakeDuration<RefSource>>(Box::new(
                sound_map::clone_sound(props.get_source("audio 1")?)?
                    .take_duration(props.get_duration("duration")?),
            )),
        },
    )]))
}
