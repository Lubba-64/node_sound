use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::daw_automation_mix::DawAutomationMix;
use crate::sounds::daw_automation_source::DawAutomationChannel;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};
pub fn daw_automation_mix_node() -> SoundNode {
    SoundNode {
        name: "Daw Automations Mix".to_string(),
        tooltip: r#"Mixes between audio 1 and audio 2 based on the daw parameter."#.to_string(),
        inputs: BTreeMap::from([
            (
                "channel".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOnly,
                    name: "channel".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: 0.0,
                        max: 17.0,
                    },
                },
            ),
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

pub fn daw_automation_mix_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned1 = props.clone_sound(props.get_source("audio 2")?)?;
    let cloned2 = props.clone_sound(props.get_source("audio 1")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(DawAutomationMix::new(
                props.state._unserializeable_state.automations.0
                    [(props.get_float("channel")?.round() as usize).clamp(0, 17)]
                .clone(),
                cloned1,
                cloned2,
            ))),
        },
    )]))
}
