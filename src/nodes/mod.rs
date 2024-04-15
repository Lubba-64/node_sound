mod mix_node;
use mix_node::mix_node;
mod sawtooth_node;
use sawtooth_node::sawtooth_node;
mod triangle_node;
use std::collections::BTreeMap;
use triangle_node::traingle_node;

use crate::sound_graph::graph_types::{InputParameter, Output, ValueType};
use std::collections::HashMap;

pub struct SoundNodeProps {
    pub inputs: HashMap<String, ValueType>,
}

impl SoundNodeProps {
    pub fn new(
        inputs: HashMap<String, ValueType>,
        output_connection_counts: HashMap<String, usize>,
    ) -> Self {
        SoundNodeProps { inputs }
    }
}

#[derive(Clone)]
pub struct SoundNode {
    pub name: String,
    pub inputs: HashMap<String, InputParameter>,
    pub outputs: HashMap<String, Output>,
    pub operation: fn(SoundNodeProps) -> HashMap<String, ValueType>,
}

pub struct NodeDefinitions(pub BTreeMap<String, SoundNode>);

pub fn get_nodes() -> NodeDefinitions {
    let nodes = [sawtooth_node(), mix_node(), traingle_node()];
    NodeDefinitions(BTreeMap::from_iter(
        nodes.iter().map(|n| (n.name.clone(), n.clone())),
    ))
}
