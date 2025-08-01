use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::MergeChannels;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};
pub fn merge_channels_node() -> SoundNode {
    SoundNode {
        name: "Merge Channels".to_string(),
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
                "audio 2".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio source 2".to_string(),
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

pub fn merge_channels_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned1 = props.clone_sound_ref(props.get_source("audio 1")?)?;
    let cloned2 = props.clone_sound_ref(props.get_source("audio 2")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(MergeChannels::new(cloned1, cloned2))),
        },
    )]))
}
