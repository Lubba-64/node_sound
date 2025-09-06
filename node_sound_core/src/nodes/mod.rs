use std::{collections::BTreeMap, time::Duration};

use eframe::egui::ahash::HashMap;
use serde::{Deserialize, Serialize};
use synthrs::midi::MidiSong;

use crate::{
    sound_graph::{
        graph::UnserializeableGraphState,
        graph_types::{InputParameter, Output, ValueType},
    },
    sound_map::{DawSource, GenericSource},
};

pub mod automated_triangle_node;
pub mod const_node;
pub mod flip_node;
pub mod lfo_node;
pub mod minus_node;
pub mod mix_node;
pub mod output_node;
pub mod sawtooth_node;
pub mod sine_node;
pub mod speed_node;
pub mod square_node;
pub mod translate_node;
pub mod triangle_node;
pub mod wave_shaper_node;
pub mod wave_table_node;
pub mod wrapper_node;

pub struct SoundNodeProps<'a> {
    pub inputs: HashMap<String, ValueType>,
    pub state: &'a mut UnserializeableGraphState,
}

impl<'a> SoundNodeProps<'a> {
    fn push_sound(&mut self, sound: Box<dyn DawSource>) -> usize {
        self.state.queue.push_sound(sound)
    }

    fn clone_sound(&mut self, idx: usize) -> Result<GenericSource, Box<dyn std::error::Error>> {
        self.state.queue.clone_sound(idx)
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
        (
            sawtooth_node::sawtooth_node(),
            Box::new(sawtooth_node::sawtooth_logic),
        ),
        (sine_node::sine_node(), Box::new(sine_node::sine_logic)),
        (
            square_node::square_node(),
            Box::new(square_node::square_logic),
        ),
        (
            triangle_node::triangle_node(),
            Box::new(triangle_node::triangle_logic),
        ),
        (mix_node::mix_node(), Box::new(mix_node::mix_logic)),
        (minus_node::minus_node(), Box::new(minus_node::minus_logic)),
        (const_node::const_node(), Box::new(const_node::const_logic)),
        (speed_node::speed_node(), Box::new(speed_node::speed_logic)),
        (lfo_node::lfo_node(), Box::new(lfo_node::lfo_logic)),
        (flip_node::flip_node(), Box::new(flip_node::flip_logic)),
        (
            output_node::output_node(),
            Box::new(output_node::output_logic),
        ),
        (
            wrapper_node::wrapper_node(),
            Box::new(wrapper_node::wrapper_logic),
        ),
        (
            wave_table_node::wave_table_node(),
            Box::new(wave_table_node::wave_table_logic),
        ),
        (
            wave_shaper_node::wave_shaper_node(),
            Box::new(wave_shaper_node::wave_shaper_logic),
        ),
        (
            translate_node::translate_node(),
            Box::new(translate_node::translate_logic),
        ),
        (
            automated_triangle_node::automated_triangle_node(),
            Box::new(automated_triangle_node::automated_triangle_logic),
        ),
    ];
    NodeDefinitions(BTreeMap::from_iter(
        nodes.iter().map(|n| (n.0.name.clone(), n.clone())),
    ))
}
