use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::automated_square::AutomatedSquareWave;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};

pub fn automated_square_node() -> SoundNode {
    SoundNode {
        name: "Automated Square Wave".to_string(),
        tooltip: r#"Automated version of the Square node.
Automates the frequency with a given waveform.
Use TranslateWave to set the frequency values of the automation,
by setting the end min and end max to your desired frequency values."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "freq".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "freq".to_string(),
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

pub fn automated_square_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("freq")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(AutomatedSquareWave::new(
                cloned,
                props.get_bool("note independant")?,
            ))),
        },
    )]))
}
