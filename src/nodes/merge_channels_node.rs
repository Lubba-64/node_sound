use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_queue;
use crate::sounds::{AsGenericSource, MergeChannels};
use egui_node_graph_2::InputParamKind;
use std::collections::HashMap;

use super::{SoundNodeProps, SoundNodeResult};
pub fn merge_channels_node() -> SoundNode {
    SoundNode {
        name: "Merge Channels".to_string(),
        inputs: HashMap::from([
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
                "audio 2".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio source 2".to_string(),
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

pub fn merge_channels_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(HashMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_queue::push_sound(
                MergeChannels::new(
                    sound_queue::clone_sound(props.get_source("audio 1")?)?,
                    sound_queue::clone_sound(props.get_source("audio 2")?)?,
                )
                .as_generic(None),
            ),
        },
    )]))
}
