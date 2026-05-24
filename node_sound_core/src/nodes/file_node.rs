use crate::{
    node_prelude::*,
    nodes::wave_table::{WaveTableManager, WaveTableOscillator},
};
use rodio::Source;
use rodio::{Decoder, source::UniformSourceIterator};
use std::io::Cursor;

#[derive(Clone, Debug)]
pub struct CloneableDecoder {
    pub wavetable: WaveTableOscillator,
}

impl CloneableDecoder {
    pub fn new(
        data: Vec<u8>,
        uses_speed: bool,
        sample_rate: u32,
        speed: f32,
        manager: &mut WaveTableManager,
    ) -> Self {
        Self {
            wavetable: manager.make_wavetable_samples(
                sample_rate as f32,
                1.0,
                1.0,
                uses_speed,
                speed,
                Box::new(|| {
                    let data = data.clone();
                    let data: Vec<_> = UniformSourceIterator::new(
                        match Decoder::new(Cursor::new(data)) {
                            Ok(samples) => samples,
                            Err(_) => return (vec![], vec![]),
                        }
                        .convert_samples::<f32>()
                        .speed(1.0),
                        1,
                        sample_rate,
                    )
                    .collect();
                    (data.clone(), data)
                }),
            ),
        }
    }
}

impl SoundNode for CloneableDecoder {
    fn next(&mut self, index: f32, channel: u8) -> Option<f32> {
        self.wavetable.next(index, channel)
    }
}

pub fn file_node() -> SoundNodeMetadata {
    SoundNodeMetadata {
        name: "Audio File".to_string(),
        tooltip: r#"Imports a wav, flac, or mp3 file as a waveform. Mono audio preferred."#
            .to_string(),
        inputs: vec![
            Input {
                data_type: DataType::AudioFile,
                kind: InputParamKind::ConstantOnly,
                name: "file".to_string(),
                value: InputValueConfig::AudioFile {},
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
            let file = match props.get_file("file")? {
                None => {
                    return Ok(BTreeMap::from([(
                        "out".to_string(),
                        ValueType::AudioSource { value: 0 },
                    )]));
                }
                Some(file) => file,
            };
            let decoder = CloneableDecoder::new(
                file.1.clone(),
                props.get_bool("note independant")?,
                props.sample_rate() as u32,
                props.note_speed(),
                &mut props.state.user_state.wavetables,
            );
            Ok(BTreeMap::from([(
                "out".to_string(),
                ValueType::AudioSource {
                    value: props.push_sound(Box::new(decoder)),
                },
            )]))
        })),
    }
}
