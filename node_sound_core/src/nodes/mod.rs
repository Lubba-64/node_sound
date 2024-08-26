mod mix_node;
use code_node::{code_logic, code_node};
use mix_node::{mix_logic, mix_node};
mod duration_node;
mod sawtooth_node;
use duration_node::{duration_logic, duration_node};
use sawtooth_node::{sawtooth_logic, sawtooth_node};
mod sine_node;
mod triangle_node;
use serde::{Deserialize, Serialize};
use sine_node::{sine_logic, sine_node};
use std::{collections::BTreeMap, time::Duration};
use synthrs::midi::MidiSong;
use triangle_node::{triangle_logic, triangle_node};
use wave_shaper_node::{wave_shaper_logic, wave_shaper_node};
mod square_node;
use square_node::{square_logic, square_node};
mod delay_node;
use crate::sound_graph::{
    graph::VstType,
    graph_types::{InputParameter, Output, ValueType},
};
use delay_node::{delay_logic, delay_node};
use std::collections::HashMap;
mod amplify_node;
use amplify_node::{amplify_logic, amplify_node};
mod repeat_infinite;
use repeat_infinite::{repeat_infinite_logic, repeat_infinite_node};
mod speed_node;
use speed_node::{speed_logic, speed_node};
mod lfo_node;
use lfo_node::{lfo_logic, lfo_node};
mod file_node;
use file_node::{file_logic, file_node};
mod clamp_node;
use clamp_node::{clamp_logic, clamp_node};
mod abs_node;
use abs_node::{abs_logic, abs_node};
mod noise_node;
use noise_node::{noise_logic, noise_node};
mod merge_channels_node;
mod skip_node;
mod split_channels_node;
use merge_channels_node::{merge_channels_logic, merge_channels_node};
use skip_node::{skip_logic, skip_node};
use split_channels_node::{split_channels_logic, split_channels_node};
mod reverb;
use reverb::{reverb_logic, reverb_node};
mod mod_node;
use mod_node::{mod_logic, mod_node};
mod automated_sawtooth_node;
use automated_sawtooth_node::{automated_sawtooth_logic, automated_sawtooth_node};
mod translate_node;
use translate_node::{translate_logic, translate_node};
mod automated_sine_node;
use automated_sine_node::{automated_sine_logic, automated_sine_node};
mod automated_square_node;
use automated_square_node::{automated_square_logic, automated_square_node};
mod automated_triangle_node;
use automated_triangle_node::{automated_triangle_logic, automated_triangle_node};
mod signum_node;
use signum_node::{signum_logic, signum_node};
mod automated_mod_node;
use automated_mod_node::{automated_mod_logic, automated_mod_node};
mod automated_clamp;
use automated_clamp::{automated_clamp_logic, automated_clamp_node};
mod automated_translate_node;
use automated_translate_node::{automated_translate_logic, automated_translate_node};
mod const_node;
use const_node::{const_logic, const_node};
mod midi_node;
use midi_node::{midi_logic, midi_node};
mod wrapper_node;
use wrapper_node::{wrapper_logic, wrapper_node};
mod output_node;
use output_node::{output_logic, output_node};
mod daw_automation_source_node;
use daw_automation_source_node::{daw_automations_logic, daw_automations_node};
mod daw_input_node;
use daw_input_node::{daw_input_logic, daw_input_node};
mod automated_wave_shaper_node;
mod wave_shaper_node;
use automated_wave_shaper_node::{automated_wave_shaper_logic, automated_wave_shaper_node};
mod code_node;

pub struct SoundNodeProps {
    pub inputs: HashMap<String, ValueType>,
}

impl SoundNodeProps {
    fn get_float(&self, name: &str) -> Result<f32, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_float()?)
    }
    fn get_source(&self, name: &str) -> Result<usize, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_source()?)
    }
    fn get_duration(&self, name: &str) -> Result<Duration, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_duration()?)
    }
    fn get_file(
        &self,
        name: &str,
    ) -> Result<Option<(String, Vec<u8>)>, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_file()?)
    }
    fn get_midi(
        &self,
        name: &str,
    ) -> Result<Option<(String, MidiSong)>, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_midi()?)
    }
    fn get_graph(&self, name: &str) -> Result<Option<Vec<f32>>, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_graph()?)
    }
    fn get_code(&self, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_code()?)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SoundNode {
    pub name: String,
    pub inputs: BTreeMap<String, InputParameter>,
    pub outputs: BTreeMap<String, Output>,
}
type SoundNodeOp =
    fn(SoundNodeProps) -> Result<BTreeMap<String, ValueType>, Box<dyn std::error::Error>>;
type SoundNodeResult = Result<BTreeMap<String, ValueType>, Box<dyn std::error::Error>>;
pub struct NodeDefinitions(pub BTreeMap<String, (SoundNode, Box<SoundNodeOp>)>);

pub fn get_nodes(is_vst: VstType) -> NodeDefinitions {
    let mut nodes: Vec<(SoundNode, Box<SoundNodeOp>)> = vec![
        (mix_node(), Box::new(mix_logic)),
        (duration_node(), Box::new(duration_logic)),
        (delay_node(), Box::new(delay_logic)),
        (amplify_node(), Box::new(amplify_logic)),
        (repeat_infinite_node(), Box::new(repeat_infinite_logic)),
        (sine_node(), Box::new(sine_logic)),
        (sawtooth_node(), Box::new(sawtooth_logic)),
        (triangle_node(), Box::new(triangle_logic)),
        (square_node(), Box::new(square_logic)),
        (speed_node(), Box::new(speed_logic)),
        (lfo_node(), Box::new(lfo_logic)),
        (clamp_node(), Box::new(clamp_logic)),
        (abs_node(), Box::new(abs_logic)),
        (noise_node(), Box::new(noise_logic)),
        (skip_node(), Box::new(skip_logic)),
        (split_channels_node(), Box::new(split_channels_logic)),
        (merge_channels_node(), Box::new(merge_channels_logic)),
        (reverb_node(), Box::new(reverb_logic)),
        (mod_node(), Box::new(mod_logic)),
        (translate_node(), Box::new(translate_logic)),
        (
            automated_sawtooth_node(),
            Box::new(automated_sawtooth_logic),
        ),
        (automated_sine_node(), Box::new(automated_sine_logic)),
        (automated_square_node(), Box::new(automated_square_logic)),
        (
            automated_triangle_node(),
            Box::new(automated_triangle_logic),
        ),
        (automated_mod_node(), Box::new(automated_mod_logic)),
        (signum_node(), Box::new(signum_logic)),
        (automated_clamp_node(), Box::new(automated_clamp_logic)),
        (
            automated_translate_node(),
            Box::new(automated_translate_logic),
        ),
        (const_node(), Box::new(const_logic)),
        (wrapper_node(), Box::new(wrapper_logic)),
        (wave_shaper_node(), Box::new(wave_shaper_logic)),
        (
            automated_wave_shaper_node(),
            Box::new(automated_wave_shaper_logic),
        ),
        (code_node(), Box::new(code_logic)),
    ];
    match is_vst {
        VstType::Effect => {
            nodes.push((daw_input_node(), Box::new(daw_input_logic)));
            nodes.push((output_node(), Box::new(output_logic)));
            nodes.push((daw_automations_node(), Box::new(daw_automations_logic)))
        }
        VstType::None => {
            nodes.push((midi_node(), Box::new(midi_logic)));
            nodes.push((file_node(), Box::new(file_logic)));
        }
        VstType::Synth => {
            nodes.push((output_node(), Box::new(output_logic)));
            nodes.push((daw_automations_node(), Box::new(daw_automations_logic)))
        }
    }
    NodeDefinitions(BTreeMap::from_iter(
        nodes.iter().map(|n| (n.0.name.clone(), n.clone())),
    ))
}
