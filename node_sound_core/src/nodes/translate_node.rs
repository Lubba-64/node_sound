use crate::constants::MAX_FREQ;
use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TranslateWave<I: SoundNode> {
    source: I,
    start_min: f32,
    start_max: f32,
    end_min: f32,
    end_max: f32,
}

impl<I: SoundNode> TranslateWave<I> {
    #[inline]
    pub fn new(
        source: I,
        mut start_min: f32,
        mut start_max: f32,
        mut end_min: f32,
        mut end_max: f32,
    ) -> Self {
        if start_min > start_max {
            let other = start_min;
            start_min = start_max;
            start_max = other;
        }
        if end_min > end_max {
            let other = end_min;
            end_min = end_max;
            end_max = other;
        }
        Self {
            source: source,
            start_min,
            start_max,
            end_min,
            end_max,
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for TranslateWave<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        return match self.source.next(index, channel) {
            Some(p) => Some(
                self.end_min
                    + ((self.end_max - self.end_min) / (self.start_max - self.start_min))
                        * (p.clamp(self.start_min, self.start_max) - self.start_min),
            ),
            _ => None,
        };
    }
}

pub fn translate_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Translate Wave".to_string(),
        tooltip: r#"Takes a wave going from start min and start max and
morphs its position to be within the range of end min and end max."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "start_min".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "start_min".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        max: MAX_FREQ,
                        min: -MAX_FREQ,
                    },
                },
            ),
            (
                "start_max".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "start_max".to_string(),
                    value: InputValueConfig::Float {
                        value: -1.0,
                        max: MAX_FREQ,
                        min: -MAX_FREQ,
                    },
                },
            ),
            (
                "end_min".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "end_min".to_string(),
                    value: InputValueConfig::Float {
                        value: 1.0,
                        max: MAX_FREQ,
                        min: -MAX_FREQ,
                    },
                },
            ),
            (
                "end_max".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "end_max".to_string(),
                    value: InputValueConfig::Float {
                        value: -1.0,
                        max: MAX_FREQ,
                        min: -MAX_FREQ,
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
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(TranslateWave::new(
                        cloned,
                        props.get_float("start_min")?,
                        props.get_float("start_max")?,
                        props.get_float("end_min")?,
                        props.get_float("end_max")?,
                    ))),
                },
            )]))
        })),
    }
}
