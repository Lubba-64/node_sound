use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::fm_synth::FMSynth;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn fm_synth_node() -> SoundNode {
    SoundNode {
        name: "FM Synth".to_string(),
        tooltip: r#"Frequency Modulation Synth that takes in oscillators."#.to_string(),
        inputs: BTreeMap::from([
            (
                "modulation index".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "modulation index".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.001,
                        min: 0.001,
                        max: 100.0,
                    },
                },
            ),
            (
                "modulation osc".to_string(),
                InputParameter {
                    data_type: DataType::Oscillator,
                    kind: InputParamKind::ConnectionOnly,
                    name: "modulation osc".to_string(),
                    value: InputValueConfig::Oscillator {},
                },
            ),
            (
                "carrier osc".to_string(),
                InputParameter {
                    data_type: DataType::Oscillator,
                    kind: InputParamKind::ConnectionOnly,
                    name: "carrier osc".to_string(),
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

pub fn fm_synth_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let osc1 = props.clone_osc(props.get_osc("carrier osc")?)?;
    let osc2 = props.clone_osc(props.get_osc("modulation osc")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(FMSynth::new(
                osc1,
                osc2,
                props.get_float("modulation index")?,
                props.sample_rate(),
            ))),
        },
    )]))
}
