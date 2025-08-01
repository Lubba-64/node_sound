use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::RepeatRefSource;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn repeat_n_node() -> SoundNode {
    SoundNode {
        name: "Repeat N".to_string(),
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
                "n".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "n".to_string(),
                    value: InputValueConfig::Float {
                        value: 2.,
                        min: 1.,
                        max: 100.,
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

pub fn repeat_n_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let repeat_count = props.get_float("n")?.floor() as usize;
    let source = props.clone_sound(props.get_source("audio 1")?)?;

    let repeated_source = RepeatRefSource::new(source, repeat_count as u32);

    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(repeated_source)),
        },
    )]))
}
