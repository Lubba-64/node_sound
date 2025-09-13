use super::{SoundNodeProps, SoundNodeResult};
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::bit_crush::BitCrusher;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

pub fn bit_crusher_node() -> SoundNode {
    SoundNode {
        name: "Bit Crusher".to_string(),
        inputs: BTreeMap::from([
            (
                "reduction".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "reduction".to_string(),
                    value: InputValueConfig::Float {
                        value: 4.0,
                        max: 128.0,
                        min: 1.0,
                    },
                },
            ),
            (
                "audio".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio".to_string(),
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

pub fn bit_crusher_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(BitCrusher::new(
                cloned,
                props.get_float("reduction")? as u32,
            ))),
        },
    )]))
}
