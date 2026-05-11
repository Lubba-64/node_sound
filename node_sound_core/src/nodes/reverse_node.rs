use crate::{
    node_prelude::*,
    nodes::wave_table::{WaveTableManager, WaveTableOscillator},
};

#[derive(Clone, Debug)]
pub struct ReverseSource {
    wavetable: WaveTableOscillator,
}

impl ReverseSource {
    #[inline]
    pub fn new<S: SoundNode>(
        source: S,
        duration: f32,
        sample_rate: f32,
        manager: &mut WaveTableManager,
    ) -> Self {
        Self {
            wavetable: manager.make_wavetable(
                sample_rate,
                1.0,
                source,
                duration,
                1.0,
                false,
                1.0,
                Box::new(|source, total_samples| {
                    let mut left = Vec::with_capacity(total_samples);
                    let mut right = Vec::with_capacity(total_samples);
                    for i in 0..total_samples {
                        let index = i as f32;
                        left.push(source.next(index, 0).unwrap_or(0.0));
                        right.push(source.next(index, 1).unwrap_or(0.0));
                    }
                    left.reverse();
                    right.reverse();
                    (left, right)
                }),
            ),
        }
    }
}

impl SoundNode for ReverseSource {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.wavetable.get_sample(index, channel)
    }
}

pub fn reverse_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Reverse".to_string(),
        tooltip: r#"Reverses a waveform over a certain duration."#.to_string(),
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
                "duration".to_string(),
                InputParameter {
                    data_type: DataType::Duration,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "duration".to_string(),
                    value: InputValueConfig::Duration { value: 1.0 },
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
            let cloned = props.clone_sound(props.get_source("audio 1")?)?;
            let source = ReverseSource::new(
                cloned,
                props.get_duration("duration")?.as_secs_f32(),
                props.sample_rate(),
                &mut props.state.user_state.wavetables,
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(source)),
                },
            )]))
        })),
    }
}
