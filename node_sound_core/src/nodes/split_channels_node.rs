use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::SplitChannels;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};
pub fn split_channels_node() -> SoundNode {
    SoundNode {
        name: "Split Channels".to_string(),
        inputs: BTreeMap::from([
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio source 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "channel".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOnly,
                    name: "channel".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: 0.0,
                        max: 1.0,
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

pub fn split_channels_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(SplitChannels::new(
                cloned,
                props.get_float("channel")?.round() as u16,
            ))),
        },
    )]))
}
