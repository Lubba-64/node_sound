use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::RawMod;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn mod_raw_node() -> SoundNode {
    SoundNode {
        name: "Mod Raw".to_string(),
        inputs: BTreeMap::from([
            (
                "mod".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "mod".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: -1.0,
                        max: 1.0,
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
pub fn mod_raw_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound_ref(props.get_source("audio 1")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(RawMod::new(cloned, props.get_float("mod")?))),
        },
    )]))
}
