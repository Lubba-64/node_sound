use crate::constants::MAX_FREQ;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::sawtooth::SawtoothWave;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

pub fn sawtooth_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Sawtooth Wave".to_string(),
        tooltip: r#"Sawtooth waveform generator."#.to_string(),
        inputs: BTreeMap::from([
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
        op: Some(Arc::new(|mut props| {
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(SawtoothWave::new(
                        props.get_float("frequency")?,
                        props.get_bool("note independant")?,
                        props.sample_rate(),
                        props.note_speed(),
                    ))),
                },
            )]))
        })),
    }
}
