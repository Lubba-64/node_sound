use crate::constants::MAX_FREQ;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::speed::Speed;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

pub fn speed_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Speed".to_string(),
        tooltip: r#"Changes the speed of the input waveform."#.to_string(),
        inputs: BTreeMap::from([
            (
                "speed".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "speed".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        min: 0.0,
                        max: MAX_FREQ,
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
        op: Some(Arc::new(|mut props| {
            let cloned = Speed::new(
                props.clone_sound(props.get_source("audio 1")?)?,
                props.get_float("speed")?,
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(cloned)),
                },
            )]))
        })),
    }
}
