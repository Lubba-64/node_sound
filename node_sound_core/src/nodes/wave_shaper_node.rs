use crate::node_prelude::*;

pub fn wave_shaper_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Wave Shaper".to_string(),
        tooltip: r#"Shape a waveform manually."#.to_string(),
        inputs: BTreeMap::from([
            (
                "graph".to_string(),
                Input {
                    data_type: DataType::Graph,
                    kind: InputParamKind::ConstantOnly,
                    name: "graph".to_string(),
                    value: InputValueConfig::Graph {
                        value: vec![0.01; WAVE_TABLE_SIZE],
                        height: 100.0,
                        width: 500.0,
                    },
                },
            ),
            (
                "frequency".to_string(),
                Input {
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
                Input {
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
            props.update_wavetables_node_idx();
            let table: Vec<f32> = props
                .get_graph("graph")?
                .unwrap_or(vec![0.01; WAVE_TABLE_SIZE]);
            let wavetable = props.state.user_state.wavetables.make_wavetable_samples(
                props.sample_rate(),
                MIDDLE_C_FREQ,
                props.get_float("frequency")?,
                props.get_bool("note independant")?,
                props.note_speed(),
                Box::new(|| (table.clone(), table.clone())),
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(wavetable)),
                },
            )]))
        })),
    }
}
