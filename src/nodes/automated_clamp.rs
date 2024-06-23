use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map::{self, RefSource};
use crate::sounds::{AutomatedClamp, AutomatedTriangleWave};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn automated_clamp_node() -> SoundNode {
    SoundNode {
        name: "Automated Clamp Wave".to_string(),
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
                "max".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "max".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "min".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "min".to_string(),
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
pub fn automated_clamp_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound::<AutomatedClamp<RefSource, RefSource, RefSource>>(
                Box::new(AutomatedClamp::new(
                    sound_map::clone_sound(props.get_source("audio 1")?)?,
                    sound_map::clone_sound(props.get_source("min")?)?,
                    sound_map::clone_sound(props.get_source("max")?)?,
                )),
            ),
        },
    )]))
}
