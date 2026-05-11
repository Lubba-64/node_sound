use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Avg<I: SoundNode> {
    source: I,
    table: VecDeque<f32>,
    size: usize,
}

impl<I: SoundNode> Avg<I> {
    #[inline]
    pub fn new(source: I, table_size: usize) -> Self {
        Self {
            source,
            table: VecDeque::new(),
            size: table_size,
        }
    }
}

impl<I: SoundNode + Clone> SoundNode for Avg<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.table
            .push_back(self.source.next(index, channel).unwrap_or_default());
        if self.table.len() > self.size {
            self.table.pop_front();
        }
        Some(self.table.iter().map(|sample| *sample).sum::<f32>() / self.table.len() as f32)
    }
}

pub fn avg_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Avg".to_string(),
        tooltip: r#"Averages all values in a list of values determined by the length parameter.
        the values are cycled each time the table produces a sample."#
            .to_string(),
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
                "length".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOnly,
                    name: "length".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: 0.0,
                        max: 25.0,
                    },
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
                    value: props.push_sound(Box::new(Avg::new(
                        cloned,
                        props.get_float("length")? as usize,
                    ))),
                },
            )]))
        })),
    }
}
