use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map::{self, RefSource};
use egui_node_graph_2::InputParamKind;
use rodio::source::Speed;
use rodio::Source;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn speed_node() -> SoundNode {
    SoundNode {
        name: "Speed".to_string(),
        inputs: BTreeMap::from([
            (
                "speed".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "speed".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: 4000.0,
                    },
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
pub fn speed_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: sound_map::push_sound::<Speed<RefSource>>(Box::new(
                sound_map::clone_sound(props.get_source("audio 1")?)?
                    .speed(props.get_float("speed")?),
            )),
        },
    )]))
}
