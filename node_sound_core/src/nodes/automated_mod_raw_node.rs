use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::automated_mod_raw::AutomatedModRaw;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn automated_mod_raw_node() -> SoundNode {
    SoundNode {
        name: "Automated Mod Raw".to_string(),
        tooltip: r#"Automated version of the Mod Raw node.
The mod amount is controlled by a waveform going from -1.0 to 1.0.
Mod Raw uses the division remainder operator (mod) on a given wave."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "mod".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "mod".to_string(),
                    value: InputValueConfig::AudioSource {},
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
pub fn automated_mod_raw_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned1 = props.clone_sound(props.get_source("audio 1")?)?;
    let cloned2 = props.clone_sound(props.get_source("mod")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(AutomatedModRaw::new(cloned1, cloned2))),
        },
    )]))
}
