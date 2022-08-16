mod mix_node;
use mix_node::mix_node;
mod sawtooth_node;
use sawtooth_node::sawtooth_node;
mod triangle_node;
use triangle_node::traingle_node;
mod sounds;
pub use sounds::*;
use std::collections::BTreeMap;

use super::types::NodeDefinitions;

pub fn get_nodes() -> NodeDefinitions {
    let nodes = [sawtooth_node(), mix_node(), traingle_node()];
    NodeDefinitions(BTreeMap::from_iter(
        nodes.iter().map(|n| (n.name.clone(), n.clone())),
    ))
}
