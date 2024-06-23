use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map::{self, RefSource};
use crate::sounds::{AutomatedTranslateWave, AutomatedTriangleWave};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn automated_translate_node() -> SoundNode {
    SoundNode {
        name: "Automated Translate Wave".to_string(),
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
                "start_max".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "start_max".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "start_min".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "start_min".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "end_max".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "end_max".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "end_min".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "end_min".to_string(),
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
pub fn automated_translate_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound::<
                AutomatedTranslateWave<RefSource, RefSource, RefSource, RefSource, RefSource>,
            >(Box::new(AutomatedTranslateWave::new(
                sound_map::clone_sound(props.get_source("audio 1")?)?,
                sound_map::clone_sound(props.get_source("start_min")?)?,
                sound_map::clone_sound(props.get_source("start_max")?)?,
                sound_map::clone_sound(props.get_source("end_min")?)?,
                sound_map::clone_sound(props.get_source("end_max")?)?,
            ))),
        },
    )]))
}
