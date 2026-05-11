use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::automated_skip::AutomatedSkip;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

pub fn automated_skip_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Skip".to_string(),
        tooltip: r#"Skips samples in the source for a given duration."#.to_string(),
        inputs: BTreeMap::from([
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "duration".to_string(),
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
        op: Some(Arc::new(|mut props| {
            let cloned = AutomatedSkip::new(
                props.clone_sound(props.get_source("duration")?)?,
                props.clone_sound(props.get_source("audio 1")?)?,
                props.get_bool("note independant")?,
                props.sample_rate(),
                props.note_speed(),
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
