use crate::node_prelude::*;

#[derive(Clone, Debug)]
pub struct AutomatedBPMSync<S: SoundNode> {
    sample_rate: f32,
    table: Vec<f32>,
    speed: f32,
    note_speed_type: NoteSpeedType,
    note_speed: S,
    note_speed_min: f32,
    note_speed_max: f32,
    bpm: Arc<Mutex<f32>>,
}

impl<S: SoundNode + Clone> SoundNode for AutomatedBPMSync<S> {
    fn next(&mut self, mut index: f32, channel: u8) -> Option<f32> {
        let seconds_per_note = self
            .note_speed_type
            .get_beats_type()
            .iter()
            .nth(
                self.note_speed
                    .next(index, channel)
                    .map(|sample| scale_param(sample, self.note_speed_min, self.note_speed_max))
                    .unwrap_or_default() as usize,
            )
            .cloned()
            .unwrap_or_default()
            .get_beats()
            / (self.bpm.lock().map(|bpm| *bpm).unwrap_or(120.0) / 60.0);
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

pub fn automated_bpm_sync_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated BPM Sync".to_string(),
        tooltip: r#"Syncs a wavetable to each note.
The automation values for Any is 0-21, 0-7 for the rest which corresponds to the automation input value."#
            .to_string(),
        inputs: vec![
            Input {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOrConstant,
                name: "min note speed".to_string(),
                value: InputValueConfig::Float {
                    value: 0.0,
                    min: 0.0,
                    max: 21.0,
                },
            }, Input {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOrConstant,
                name: "max note speed".to_string(),
                value: InputValueConfig::Float {
                    value: 0.0,
                    min: 0.0,
                    max: 21.0,
                },
            },Input {
                data_type: DataType::Graph,
                kind: InputParamKind::ConstantOnly,
                name: "graph".to_string(),
                value: InputValueConfig::Graph {
                    value: vec![0.01; WAVE_TABLE_SIZE],
                    height: 100.0,
                    width: 300.0,
                },
            }, Input {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "note speed".to_string(),
                value: InputValueConfig::AudioSource {},
            }, Input {
                data_type: DataType::Dropdown,
                kind: InputParamKind::ConstantOnly,
                name: "note speeds".to_string(),
                value: InputValueConfig::Dropdown {
                    value: NoteSpeedType::Normal.to_string(),
                    values: NoteSpeedType::ALL.map(|speed_type| speed_type.to_string()).to_vec(),
                },
            },
        ],
        outputs: get_default_outputs(),
        op: Some(Arc::new(|mut props| {
            let speed = NoteSpeedType::from_str(&props.get_dropdown("note speed type")?)?;
            let note_speed = props.clone_sound(props.get_source("note speed")?)?;
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(AutomatedBPMSync{
                        bpm: props.bpm(),
                        note_speed,
                        note_speed_max: props.get_float("max note speed")?,
                        note_speed_min: props.get_float("min note speed")?,
                        note_speed_type: speed,
                        sample_rate: props.sample_rate(),
                        speed: props.note_speed(),
                        table: props
                            .get_graph("graph")?
                            .unwrap_or(vec![0.0; WAVE_TABLE_SIZE]),
                    })),
                },
            )]))
        })),
    }
}
