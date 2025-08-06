mod mix_node;
use mix_node::{mix_logic, mix_node};
mod duration_node;
mod sawtooth_node;
use duration_node::{duration_logic, duration_node};
use reverse_node::{reverse_logic, reverse_node};
use sawtooth_node::{sawtooth_logic, sawtooth_node};
mod sine_node;
mod triangle_node;
use serde::{Deserialize, Serialize};
use sine_node::{sine_logic, sine_node};
use std::{collections::BTreeMap, time::Duration};
use synthrs::midi::MidiSong;
use triangle_node::{triangle_logic, triangle_node};
use wave_shaper_node::{wave_shaper_logic, wave_shaper_node};
use wave_table_node::{wave_table_logic, wave_table_node};
mod square_node;
use square_node::{square_logic, square_node};
mod delay_node;
use crate::{
    sound_graph::{
        graph::SoundNodeGraphState,
        graph_types::{InputParameter, Output, ValueType},
    },
    sound_map::{GenericSource, SourceIterDynClone},
};
use delay_node::{delay_logic, delay_node};
use std::collections::HashMap;
mod amplify_node;
use amplify_node::{amplify_logic, amplify_node};
mod repeat_infinite;
use repeat_infinite::{repeat_infinite_logic, repeat_infinite_node};
mod repeat_n_node;
use repeat_n_node::{repeat_n_logic, repeat_n_node};
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
mod automated_wave_shaper_node;
mod wave_shaper_node;
use automated_wave_shaper_node::{automated_wave_shaper_logic, automated_wave_shaper_node};
mod mod_raw_node;
pub use mod_raw_node::{mod_raw_logic, mod_raw_node};
mod automated_wave_table_node;
mod reverse_node;
mod wave_table_node;
pub use automated_wave_table_node::{automated_wave_table_logic, automated_wave_table_node};
mod bit_crush_node;
pub use bit_crush_node::{bit_crusher_logic, bit_crusher_node};

pub struct SoundNodeProps<'a> {
    pub inputs: HashMap<String, ValueType>,
    pub state: &'a mut SoundNodeGraphState,
}

impl<'a> SoundNodeProps<'a> {
    fn push_sound(&mut self, sound: Box<dyn SourceIterDynClone>) -> usize {
        self.state._unserializeable_state.queue.push_sound(sound)
    }

    fn clone_sound(&mut self, idx: usize) -> Result<GenericSource, Box<dyn std::error::Error>> {
        self.state._unserializeable_state.queue.clone_sound(idx)
    }

    fn get_float(&self, name: &str) -> Result<f32, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_float()?)
    }
    fn get_bool(&self, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self
            .inputs
            .get(name)
            .unwrap_or_default()
            .clone()
            .try_to_bool()?)
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

#[derive(Clone)]
pub struct NodeDefinitions(pub BTreeMap<String, (SoundNode, Box<SoundNodeOp>)>);

impl Default for NodeDefinitions {
    fn default() -> Self {
        get_nodes()
    }
}

pub fn get_nodes() -> NodeDefinitions {
    let nodes: Vec<(SoundNode, Box<SoundNodeOp>)> = vec![
        (mix_node(), Box::new(mix_logic)),
        (duration_node(), Box::new(duration_logic)),
        (delay_node(), Box::new(delay_logic)),
        (amplify_node(), Box::new(amplify_logic)),
        (repeat_infinite_node(), Box::new(repeat_infinite_logic)),
        (repeat_n_node(), Box::new(repeat_n_logic)),
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
        (mod_raw_node(), Box::new(mod_raw_logic)),
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
        (
            automated_wave_table_node(),
            Box::new(automated_wave_table_logic),
        ),
        (wave_table_node(), Box::new(wave_table_logic)),
        (reverse_node(), Box::new(reverse_logic)),
        (bit_crusher_node(), Box::new(bit_crusher_logic)),
        (output_node(), Box::new(output_logic)),
        (daw_automations_node(), Box::new(daw_automations_logic)),
        (midi_node(), Box::new(midi_logic)),
        (file_node(), Box::new(file_logic)),
    ];
    NodeDefinitions(BTreeMap::from_iter(
        nodes.iter().map(|n| (n.0.name.clone(), n.clone())),
    ))
}
