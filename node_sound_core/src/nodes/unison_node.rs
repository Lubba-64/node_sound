use super::{SoundNodeProps, SoundNodeResult};
use crate::constants::MAX_FREQ;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::unison::UnisonVoice;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

pub fn unison_node() -> SoundNode {
    SoundNode {
        name: "Unison".to_string(),
        tooltip: r#"Unison with multiple voices.
voices is the number of voices.
unison is the amount of unison the voices have with them being identical at 0 and
totally phase separated at 100.0"#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "unison".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "unison".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: 100.0,
                    },
                },
            ),
            (
                "voices".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "voices".to_string(),
                    value: InputValueConfig::Float {
                        value: 5.0,
                        min: 0.0,
                        max: 25.0,
                    },
                },
            ),
            (
                "frequency".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "frequency".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: 0.0,
                        max: MAX_FREQ,
                    },
                },
            ),
            (
                "detune".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "detune".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: 0.0,
                        max: 20.0,
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

pub fn unison_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(UnisonVoice::new(
                cloned,
                props.get_float("unison")?,
                props.get_float("voices")? as u8,
                props.sample_rate(),
                props.note_speed(),
                props.get_float("frequency")?,
                props.get_float("detune")?,
            ))),
        },
    )]))
}
