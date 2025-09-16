use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::abs::Abs;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn abs_node() -> SoundNode {
    SoundNode {
        name: "Abs".to_string(),
        tooltip: r#"Applies absolute value to the waveform,
bringing everything on the bottom of the waveform to the top."#
            .to_string(),
        inputs: BTreeMap::from([(
            "audio 1".to_string(),
            InputParameter {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "audio 1".to_string(),
                value: InputValueConfig::AudioSource {},
            },
        )]),
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
    }
}
pub fn abs_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(Abs::new(cloned))),
        },
    )]))
}
