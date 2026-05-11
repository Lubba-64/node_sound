use crate::node_prelude::*;

pub fn automated_wave_shaper_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Wave Shaper".to_string(),
        tooltip: r#"Automated version of the Wave Shaper node.
Automates the frequency with a given waveform.
Use TranslateWave to set the frequency values of the automation,
by setting the end min and end max to your desired frequency values."#
            .to_string(),
        inputs: BTreeMap::from([
            (
                "graph".to_string(),
                InputParameter {
                    data_type: DataType::Graph,
                    kind: InputParamKind::ConstantOnly,
                    name: "graph".to_string(),
                    value: InputValueConfig::Graph {
                        value: vec![0.0; WAVE_TABLE_SIZE],
                        height: 100.0,
                        width: 500.0,
                    },
                },
            ),
            (
                "frequency".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "frequency".to_string(),
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
            let cloned = props.clone_sound(props.get_source("frequency")?)?;
            props.update_wavetables_node_idx();
            let left = props
                .get_graph("graph")?
                .unwrap_or(vec![0.01; WAVE_TABLE_SIZE]);
            let wavetable = props
                .state
                .user_state
                .wavetables
                .make_automated_wavetable_samples(
                    props.sample_rate(),
                    MIDDLE_C_FREQ,
                    cloned,
                    props.get_bool("note independant")?,
                    props.note_speed(),
                    Box::new(|| (left.clone(), left.clone())),
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
