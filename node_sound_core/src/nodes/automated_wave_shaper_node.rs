use crate::constants::{DEFAULT_SAMPLE_RATE, MIDDLE_C_FREQ, WAVE_TABLE_SIZE};
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::automated_wave_table::AutomatedWaveTableOscillator;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn automated_wave_shaper_node() -> SoundNode {
    SoundNode {
        name: "Automated Wave Shaper".to_string(),
        tooltip: r#"Automated version of the Wave Shaper node.
Automates the frequency with a given waveform.
Use TranslateWave to set the frequency values of the automation,
by setting the end min and end max to your desired frequency values."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "graph".to_string(),
                InputParameter {
                    data_type: DataType::Graph,
                    kind: InputParamKind::ConstantOnly,
                    name: "graph".to_string(),
                    value: InputValueConfig::Graph {
                        value: vec![0.0; WAVE_TABLE_SIZE],
                        height: 100.0,
                        width: 500.0,
                    },
                },
            ),
            (
                "frequency".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "frequency".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "note independant".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "note independant".to_string(),
                    value: InputValueConfig::Bool { value: false },
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

pub fn automated_wave_shaper_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("frequency")?)?;
    let mut stereo =
        AutomatedWaveTableOscillator::new_stereo(DEFAULT_SAMPLE_RATE as f32, cloned, MIDDLE_C_FREQ);
    stereo.set_uses_speed(props.get_bool("note independant")?);
    stereo.rebuild_table(
        DEFAULT_SAMPLE_RATE as f32,
        props
            .get_graph("graph")?
            .unwrap_or(vec![0.01; WAVE_TABLE_SIZE]),
        props
            .get_graph("graph")?
            .unwrap_or(vec![0.01; WAVE_TABLE_SIZE]),
    );
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(stereo)),
        },
    )]))
}
