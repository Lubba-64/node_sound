use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Glitch<I: SoundNode> {
    current_source: I,
    ind_min: f32,
}

impl<I: SoundNode + Clone> Glitch<I> {
    #[inline]
    pub fn new(source: I) -> Self {
        Self {
            current_source: source,
            ind_min: 0.0,
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Glitch<I> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        index += 0.1;
        if index > 0.1 + 0.1 {
            self.ind_min += index - 0.1;
        }
        index -= self.ind_min;
        self.current_source.next(index, channel)
    }
}

pub fn glitch_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Glitch".to_string(),
        tooltip: r#"Glitch sounds."#.to_string(),
        inputs: BTreeMap::from([(
            "audio 1".to_string(),
            InputParameter {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "audio 1".to_string(),
                value: InputValueConfig::AudioSource {},
            },
        )]),
        outputs: BTreeMap::from([(
            "out".to_string(),
            Output {
                data_type: DataType::AudioSource,
                name: "out".to_string(),
            },
        )]),
        op: Some(Arc::new(|mut props| {
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(Glitch::new(cloned))),
                },
            )]))
        })),
    }
}
