use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sound_map::{self, RefSource};

use egui_node_graph_2::InputParamKind;
use rodio::source::Mix;
use rodio::Source;
use std::collections::BTreeMap;

use super::{SoundNodeProps, SoundNodeResult};
pub fn mix_node() -> SoundNode {
    SoundNode {
        name: "Mix".to_string(),
        inputs: BTreeMap::from([
            (
                "audio 1".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio source 1".to_string(),
                    value: InputValueConfig::AudioSource {},
                },
            ),
            (
                "audio 2".to_string(),
                InputParameter {
                    data_type: DataType::AudioSource,
                    kind: InputParamKind::ConnectionOnly,
                    name: "audio source 2".to_string(),
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
    }
}

pub fn mix_logic(props: SoundNodeProps) -> SoundNodeResult {
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
<<<<<<< HEAD
            value: sound_map::push_sound(Box::new(
=======
            value: sound_map::push_sound::<Mix<RefSource, RefSource>>(Box::new(
>>>>>>> 407096b06a510eb51621aa9b03664926c7d68c0a
                (sound_map::clone_sound_ref(props.get_source("audio 1")?)?)
                    .mix(sound_map::clone_sound_ref(props.get_source("audio 2")?)?),
            )),
        },
    )]))
}
