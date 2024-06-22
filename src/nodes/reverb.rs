use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map::{self, RefSource};
use egui_node_graph_2::InputParamKind;
use rodio::source::{Amplify, Delay, Mix};
use rodio::Source;
use std::collections::BTreeMap;
use std::time::Duration;

use super::{SoundNodeProps, SoundNodeResult};

pub fn reverb_node() -> SoundNode {
    SoundNode {
        name: "Reverb".to_string(),
        inputs: BTreeMap::from([
            (
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "duration".to_string(),
                    value: InputValueConfig::Float { value: 1.0 },
                },
            ),
            (
                "amplification".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "amplification".to_string(),
                    value: InputValueConfig::Float { value: 1.0 },
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
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}
pub fn reverb_logic(props: SoundNodeProps) -> SoundNodeResult {
    let duration = Duration::from_millis(props.get_float("duration")?.round() as u64);
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound::<Mix<RefSource, Delay<Amplify<RefSource>>>>(Box::new(
                sound_map::clone_sound(props.get_source("audio 1")?)?
                    .reverb(duration.clone(), props.get_float("amplification")?),
            )),
        },
    )]))
}
