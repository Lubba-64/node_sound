use crate::constants::{DEFAULT_SAMPLE_RATE, MAX_FREQ, MIDDLE_C_FREQ, WAVE_TABLE_SIZE};
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::wave_table::WaveTableOscillator;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn wave_shaper_node() -> SoundNode {
    SoundNode {
        name: "Wave Shaper".to_string(),
        tooltip: r#"Shape a waveform manually."#.to_string(),
        inputs: BTreeMap::from([
            (
                "graph".to_string(),
                InputParameter {
                    data_type: DataType::Graph,
                    kind: InputParamKind::ConstantOnly,
                    name: "graph".to_string(),
                    value: InputValueConfig::Graph {
                        value: vec![0.01; WAVE_TABLE_SIZE],
                        height: 100.0,
                        width: 500.0,
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
pub fn wave_shaper_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let mut audio = WaveTableOscillator::new_stereo(DEFAULT_SAMPLE_RATE as f32, MIDDLE_C_FREQ);
    audio.set_uses_speed(props.get_bool("note independant")?);
    audio.rebuild_table(
        DEFAULT_SAMPLE_RATE as f32,
        props
            .get_graph("graph")?
            .unwrap_or(vec![0.01; WAVE_TABLE_SIZE]),
        props
            .get_graph("graph")?
            .unwrap_or(vec![0.01; WAVE_TABLE_SIZE]),
    );
    audio.set_frequency(props.get_float("frequency")?);
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(audio)),
        },
    )]))
}
