use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_queue;
use crate::sounds::{AsGenericSource, SplitChannels};
use egui_node_graph_2::InputParamKind;
use rodio::source::ChannelVolume;
use std::collections::HashMap;

use super::{SoundNodeProps, SoundNodeResult};
pub fn split_channels_node() -> SoundNode {
    SoundNode {
        name: "Split Channels".to_string(),
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
                "channel".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOnly,
                    name: "channel".to_string(),
                    value: InputValueConfig::Float { value: 0.0 },
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

pub fn split_channels_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(HashMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_queue::push_sound(
                SplitChannels::new(
                    sound_queue::clone_sound(props.get_source("audio 1")?)?,
                    props.get_float("channel")?.round() as u16,
                )
                .as_generic(None),
            ),
        },
    )]))
}
