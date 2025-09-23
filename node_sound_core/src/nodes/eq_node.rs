use super::{SoundNodeProps, SoundNodeResult};
use crate::constants::MAX_FREQ;
use crate::nodes::SoundNode;
use crate::sound_graph::graph_types::{
    DataType, InputParameter, InputValueConfig, Output, ValueType,
};
use crate::sounds::eq::{FilterType, SingleFilterEq};
use egui_node_graph_2::InputParamKind;
use std::collections::BTreeMap;
use std::str::FromStr;

pub fn eq_node() -> SoundNode {
    SoundNode {
        name: "Eq".to_string(),
        tooltip: r#"Basic runtime EQ.
        Q factor controls how "selective" or "narrow" the filter is around the cutoff frequency.
        Low / High shelf and peak are the only settings that use gain. gain is in DB.
        it boosts / cuts the selected frequencies.
        Low pass filter removes frequencies above frequency.
        High pass filter removes frequencies below frequency.
        Band pass filter removes frequencies outside around frequency.
        Notch filter removes frequencies inside around frequency.
        Low shelf is an agressive low pass that cuts to 0 instead of moving smoothly.
        High shelf is an agressive high pass that cuts to 0 instead of moving smoothly.
        Peak EQ is very similar to Band pass, but width is controlled by q factor.
        "#
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
                "filter type".to_string(),
                InputParameter {
                    data_type: DataType::Dropdown,
                    kind: InputParamKind::ConstantOnly,
                    name: "filter type".to_string(),
                    value: InputValueConfig::Dropdown {
                        value: FilterType::LowPass.to_string(),
                        values: FilterType::ALL.map(|x| x.to_string()).to_vec(),
                    },
                },
            ),
            (
                "frequency".to_string(),
                InputParameter {
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
                "q factor".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "q factor".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.7,
                        min: 0.7,
                        max: 10.0,
                    },
                },
            ),
            (
                "gain".to_string(),
                InputParameter {
                    data_type: DataType::Float,
                    kind: InputParamKind::ConnectionOrConstant,
                    name: "gain".to_string(),
                    value: InputValueConfig::Float {
                        value: 0.0,
                        min: -12.0,
                        max: 12.0,
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
    }
}
pub fn eq_logic(mut props: SoundNodeProps) -> SoundNodeResult {
    let cloned = props.clone_sound(props.get_source("audio 1")?)?;
    let filter_type = FilterType::from_str(&props.get_dropdown("filter type")?)?;
    Ok(BTreeMap::from([(
        "out".to_string(),
        ValueType::AudioSource {
            value: props.push_sound(Box::new(SingleFilterEq::new(
                cloned,
                props.sample_rate(),
                2,
                filter_type,
                props.get_float("frequency")?,
                props.get_float("q factor")?,
                props.get_float("gain")?,
            ))),
        },
    )]))
}
