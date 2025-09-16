use crate::constants::WAVE_TABLE_SIZE;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::vertical_wave_shaper::VerticalWaveShaper;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn vertical_wave_shaper_node() -> SoundNode {
    SoundNode {
        name: "Vertical Wave Shaper".to_string(),
        tooltip: r#"Shapes the incoming wave vertically by the graph."#.to_string(),
        inputs: BTreeMap::from([
            (
                "graph".to_string(),
                InputParameter {
                    data_type: DataType::Graph,
                    kind: InputParamKind::ConstantOnly,
                    name: "graph".to_string(),
                    value: InputValueConfig::Graph {
                        value: vec![0.0; WAVE_TABLE_SIZE],
                        height: 200.0,
                        width: 200.0,
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

pub fn vertical_wave_shaper_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;

    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(VerticalWaveShaper::new(
                cloned,
                props
                    .get_graph("graph")?
                    .unwrap_or(vec![0.01; WAVE_TABLE_SIZE]),
            ))),
        },
    )]))
}
