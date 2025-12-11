use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::clamp_to_note::ClampToNote;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn clamp_to_note_node() -> SoundNode {
    SoundNode {
        name: "Clamp To Note".to_string(),
        tooltip: r#"Clamps the incoming value to the nearest note value. Should only be used after translate wave."#
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
pub fn clamp_to_note_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(ClampToNote::new(cloned))),
        },
    )]))
}
