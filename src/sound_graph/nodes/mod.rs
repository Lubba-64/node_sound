mod sawtooth_node;
mod sounds;
use std::collections::{BTreeMap, HashMap};

pub use sawtooth_node::sawtooth_node;
pub use sounds::*;

use super::types::NodeDefinitions;

pub fn get_nodes() -> NodeDefinitions {
    let nodes = [sawtooth_node()];
    NodeDefinitions(BTreeMap::from_iter(
        nodes.iter().map(|n| (n.name.clone(), n.clone())),
    ))
}
