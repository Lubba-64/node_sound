mod mix_node;
use mix_node::mix_node;
mod duration_node;
mod sawtooth_node;
use duration_node::duration_node;
use sawtooth_node::sawtooth_node;
mod sine_node;
mod triangle_node;
use sine_node::sine_node;
use std::{collections::BTreeMap, time::Duration};
use triangle_node::traingle_node;
mod square_node;
use square_node::square_node;
mod delay_node;
use crate::sound_graph::graph_types::{InputParameter, Output, ValueType};
use delay_node::delay_node;
use std::collections::HashMap;
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
}

#[derive(Clone)]
pub struct SoundNode {
    pub name: String,
    pub inputs: HashMap<String, InputParameter>,
    pub outputs: HashMap<String, Output>,
    pub operation:
        fn(SoundNodeProps) -> Result<HashMap<String, ValueType>, Box<dyn std::error::Error>>,
}

pub struct NodeDefinitions(pub BTreeMap<String, SoundNode>);

pub fn get_nodes() -> NodeDefinitions {
    let nodes = [
        sawtooth_node(),
        mix_node(),
        traingle_node(),
        duration_node(),
        sine_node(),
        square_node(),
        delay_node(),
    ];
    NodeDefinitions(BTreeMap::from_iter(
        nodes.iter().map(|n| (n.name.clone(), n.clone())),
    ))
}
