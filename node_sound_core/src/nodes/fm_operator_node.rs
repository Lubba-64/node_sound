use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::fm_operator::FmOperator;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn fm_operator_node() -> SoundNode {
    SoundNode {
        name: "FM Operator".to_string(),
        tooltip: "FM modulation operator with feedback and velocity sensitivity".to_string(),
        inputs: BTreeMap::from([
            (
                "volume".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "volume".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: 1.0,
                    },
                },
            ),
            (
                "mix out".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "mix out".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.5,
                        min: 0.0,
                        max: 1.0,
                    },
                },
            ),
            (
                "mod out".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "mod out".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.7,
                        min: 0.0,
                        max: 1.0,
                    },
                },
            ),
            (
                "feedback".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "feedback".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.3,
                        min: 0.0,
                        max: 1.0,
                    },
                },
            ),
            (
                "panning".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "panning".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.5,
                        min: 0.0,
                        max: 1.0,
                    },
                },
            ),
            (
                "envelope volume".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "envelope volume".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: 1.0,
                    },
                },
            ),
            (
                "vel sens mod".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "vel sens mod".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.2,
                        min: 0.0,
                        max: 1.0,
                    },
                },
            ),
            (
                "vel sens feedback".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "vel sens feedback".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.1,
                        min: 0.0,
                        max: 1.0,
                    },
                },
            ),
            (
                "key velocity".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "key velocity".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: 1.0,
                    },
                },
            ),
            (
                "mod input 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "mod input 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "mod input 2".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "mod input 2".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "oscillator".to_string(),
                InputParameter {
                    data_type: DataType::Oscillator,
                    kind: InputParamKind::ConnectionOnly,
                    name: "oscillator".to_string(),
                    value: InputValueConfig::Oscillator {},
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

pub fn fm_operator_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let oscillator = props.clone_osc(props.get_osc("oscillator")?)?;
    let volume = props.get_float("volume")?;
    let mix_out = props.get_float("mix out")?;
    let mod_out = props.get_float("mod out")?;
    let feedback = props.get_float("feedback")?;
    let panning = props.get_float("panning")?;
    let envelope_volume = props.get_float("envelope volume")?;
    let vel_sens_mod = props.get_float("vel sens mod")?;
    let vel_sens_feedback = props.get_float("vel sens feedback")?;
    let key_velocity = props.get_float("key velocity")?;

    // Get modulation inputs (optional)
    let mod_input_1 = props.clone_sound(props.get_source("mod input 1")?)?;
    let mod_input_2 = props.clone_sound(props.get_source("mod input 2")?)?;

    // Create the ModSource
    let mod_source = FmOperator::new(
        oscillator,
        volume,
        mix_out,
        mod_out,
        feedback,
        panning,
        envelope_volume,
        vel_sens_mod,
        vel_sens_feedback,
        props.sample_rate(),
        key_velocity,
        mod_input_1,
        mod_input_2,
    );
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(mod_source)),
        },
    )]))
}
