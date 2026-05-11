use crate::constants::WAVE_TABLE_SIZE;
use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct VerticalWaveShaper<I: SoundNode> {
    source: I,
    table: Vec<f32>,
}

impl<I: SoundNode> VerticalWaveShaper<I> {
    #[inline]
    pub fn new(source: I, mut table: Vec<f32>) -> Self {
        table = table.iter().map(|sample| (sample + 1.0) / 2.0).collect();
        Self { source, table }
    }
}

impl<I: SoundNode + Clone> SoundNode for VerticalWaveShaper<I> {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.source.next(index, channel).map(|sample| {
            let real_idx = sample.abs().clamp(0.0, 1.0) * (self.table.len() - 1) as f32;
            let idx = real_idx.floor() as usize;
            let initial_weight = real_idx - idx as f32;
            let initial = self.table[idx] * initial_weight;
            let second = if idx + 1 >= self.table.len() {
                self.table[idx] + 0.001
            } else {
                self.table[idx + 1]
            } * (1.0 - initial_weight);
            (initial + second) * sample.signum()
        })
    }
}

pub fn vertical_wave_shaper_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
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
        op: Some(Arc::new(|mut props| {
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
        })),
    }
}
