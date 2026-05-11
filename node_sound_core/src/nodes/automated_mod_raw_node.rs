use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AutomatedModRaw<I1: SoundNode, I2: SoundNode> {
    source: I1,
    mod_by: I2,
}

impl<I1: SoundNode, I2: SoundNode> AutomatedModRaw<I1, I2> {
    #[inline]
    pub fn new(source: I1, mod_by: I2) -> Self {
        Self { source, mod_by }
    }
}

impl<I1: SoundNode + Clone, I2: SoundNode + Clone> SoundNode for AutomatedModRaw<I1, I2> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        match (
            self.source.next(index, channel),
            self.mod_by.next(index, channel),
        ) {
            (Some(sample), Some(mod_by)) => Some(sample % mod_by),
            _ => None,
        }
    }
}

pub fn automated_mod_raw_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Mod Raw".to_string(),
        tooltip: r#"Automated version of the Mod Raw node.
The mod amount is controlled by a waveform going from -1.0 to 1.0.
Mod Raw uses the division remainder operator (mod) on a given wave."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "mod".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "mod".to_string(),
                    value: InputValueConfig::AudioSource {},
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
            let cloned1 = props.clone_sound(props.get_source("audio 1")?)?;
            let cloned2 = props.clone_sound(props.get_source("mod")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(AutomatedModRaw::new(cloned1, cloned2))),
                },
            )]))
        })),
    }
}
