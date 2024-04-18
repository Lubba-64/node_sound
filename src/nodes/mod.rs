mod mix_node;
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
use triangle_node::{triangle_logic, triangle_node};
mod square_node;
use square_node::{square_logic, square_node};
mod delay_node;
use crate::sound_graph::graph_types::{InputParameter, Output, ValueType};
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

#[derive(Clone, Serialize, Deserialize)]
pub struct SoundNode {
    pub name: String,
    pub inputs: HashMap<String, InputParameter>,
    pub outputs: HashMap<String, Output>,
}
type SoundNodeOp =
    fn(SoundNodeProps) -> Result<HashMap<String, ValueType>, Box<dyn std::error::Error>>;
type SoundNodeResult = Result<HashMap<String, ValueType>, Box<dyn std::error::Error>>;
pub struct NodeDefinitions(pub BTreeMap<String, (SoundNode, Box<SoundNodeOp>)>);

pub fn get_nodes() -> NodeDefinitions {
    let nodes: [(SoundNode, Box<SoundNodeOp>); 11] = [
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
    ];
    NodeDefinitions(BTreeMap::from_iter(
        nodes.iter().map(|n| (n.0.name.clone(), n.clone())),
    ))
}
