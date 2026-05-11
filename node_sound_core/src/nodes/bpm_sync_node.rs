use crate::constants::WAVE_TABLE_SIZE;
use crate::node::SoundNode;
use crate::nodes::SoundNodeMetadata;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_graph::note::NoteSpeed;
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct BPMSync {
    sample_rate: f32,
    table: Vec<f32>,
    speed: f32,
    note_speed: NoteSpeed,
    bpm: Arc<Mutex<f32>>,
}

impl BPMSync {
    #[inline]
    pub fn new(
        sample_rate: f32,
        bpm: Arc<Mutex<f32>>,
        note_speed: NoteSpeed,
        table: Vec<f32>,
        speed: f32,
    ) -> Self {
        Self {
            sample_rate,
            bpm,
            note_speed,
            table,
            speed,
        }
    }
}

impl SoundNode for BPMSync {
    fn next(&mut self, mut index: f32, _channel: u8) -> Option<f32> {
        let seconds_per_note =
            self.note_speed.get_beats() / (self.bpm.lock().map(|bpm| *bpm).unwrap_or(120.0) / 60.0);
        let samples_per_note = seconds_per_note * self.sample_rate;
        index /= self.speed;
        index %= samples_per_note;
        index /= samples_per_note;
        let real_idx = index * self.table.len() as f32;
        let idx = real_idx.floor() as usize;
        let initial_weight = real_idx - idx as f32;
        let first = self.table[idx] * initial_weight;
        let second = if idx + 1 >= self.table.len() {
            self.table[idx] + 0.001
        } else {
            self.table[idx + 1]
        } * (1.0 - initial_weight);
        Some(first + second)
    }
}

pub fn bpm_sync_source_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "BPM Sync (New)".to_string(),
        tooltip: r#"syncs a wavetable to each note. use"#.to_string(),
        inputs: BTreeMap::from([
            (
                "graph".to_string(),
                InputParameter {
                    data_type: DataType::Graph,
                    kind: InputParamKind::ConstantOnly,
                    name: "graph".to_string(),
                    value: InputValueConfig::Graph {
                        value: vec![0.01; WAVE_TABLE_SIZE],
                        height: 100.0,
                        width: 300.0,
                    },
                },
            ),
            (
                "note speed".to_string(),
                InputParameter {
                    data_type: DataType::Dropdown,
                    kind: InputParamKind::ConstantOnly,
                    name: "note speed".to_string(),
                    value: InputValueConfig::Dropdown {
                        value: NoteSpeed::Quarter.to_string(),
                        values: NoteSpeed::ALL.map(|speed| speed.to_string()).to_vec(),
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
            let speed = NoteSpeed::from_str(&props.get_dropdown("note speed")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(BPMSync::new(
                        props.sample_rate(),
                        props.bpm(),
                        speed,
                        props
                            .get_graph("graph")?
                            .unwrap_or(vec![0.0; WAVE_TABLE_SIZE]),
                        props.note_speed(),
                    ))),
                },
            )]))
        })),
    }
}
