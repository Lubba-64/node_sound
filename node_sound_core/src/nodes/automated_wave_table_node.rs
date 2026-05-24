use crate::node_prelude::*;

pub fn automated_wave_table_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Automated Wave Table".to_string(),
        tooltip: r#"Automated version of the Wave Table node.
Automates the frequency with a given waveform.
Use TranslateWave to set the frequency values of the automation,
by setting the end min and end max to your desired frequency values."#
            .to_string(),
        inputs: vec![
            Input {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "audio 1".to_string(),
                value: InputValueConfig::AudioSource {},
            },
            Input {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "duration".to_string(),
                value: InputValueConfig::Duration { value: 1.0 },
            },
            Input {
                data_type: DataType::AudioSource,
                kind: InputParamKind::ConnectionOnly,
                name: "frequency".to_string(),
                value: InputValueConfig::AudioSource {},
            },
            Input {
                data_type: DataType::Float,
                kind: InputParamKind::ConnectionOrConstant,
                name: "note independant".to_string(),
                value: InputValueConfig::Bool { value: false },
            },
        ],
        outputs: get_default_outputs(),
        op: Some(Arc::new(|mut props| {
            props.update_wavetables_node_idx();
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            let freq = props.clone_sound(props.get_source("frequency")?)?;
            let table = props
                .state
                .user_state
                .wavetables
                .make_automated_wavetable_generic(
                    props.sample_rate(),
                    MIDDLE_C_FREQ,
                    cloned,
                    props.get_duration("duration")?.as_secs_f32(),
                    freq,
                    props.get_bool("note independant")?,
                    props.note_speed(),
                );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(table)),
                },
            )]))
        })),
    }
}
