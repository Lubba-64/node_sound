use eframe::egui::{Pos2, Vec2};
pub use egui_node_graph_2::*;
pub use rodio::source::Zero;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    graph::{NodeData, SoundGraphEditorState},
    graph_types::{DataType, ValueType},
};
use itertools::Itertools;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct ClipboardData {
    nodes: Vec<(Node<NodeData>, Pos2)>,
    connections: Vec<(InputId, OutputId)>,
    input_params: HashMap<InputId, InputParam<DataType, ValueType>>,
    output_params: HashMap<OutputId, OutputParam<DataType>>,
}

pub fn delete_selected_nodes(state: &mut SoundGraphEditorState) {
    for node_id in state.selected_nodes.iter() {
        state.graph.remove_node(*node_id);
    }
    state.selected_nodes = vec![];
}

pub fn copy_to_clipboard(state: &mut SoundGraphEditorState) {
    let mut clipboard_data = ClipboardData {
        connections: vec![],
        nodes: vec![],
        input_params: HashMap::new(),
        output_params: HashMap::new(),
    };
    for node_id in state.selected_nodes.clone() {
        let node_data = state.graph.nodes.get(node_id);
        let node = match node_data {
            Some(x) => x.clone(),
            None => {
                continue;
            }
        };
        clipboard_data
            .nodes
            .push((node.clone(), state.node_positions[node_id].clone()));
        for input_id in node.inputs.iter().map(|(_, id)| id) {
            let output_id = state.graph.connections.get(*input_id);
            match output_id {
                Some(x) => {
                    clipboard_data.connections.push((*input_id, *x));
                }
                None => {}
            }
        }
        for input_id in node.inputs.iter().map(|(_, id)| id) {
            clipboard_data.input_params.insert(
                *input_id,
                state.graph.inputs.get(*input_id).expect("bruh").clone(),
            );
        }
        for output_id in node.outputs.iter().map(|(_, id)| id) {
            clipboard_data.output_params.insert(
                *output_id,
                state.graph.outputs.get(*output_id).expect("bruh").clone(),
            );
        }
    }

    clipboard_data.connections = clipboard_data
        .connections
        .iter()
        .unique()
        .cloned()
        .collect();

    let mut clipboard = clippers::Clipboard::get();
    clipboard
        .write_text(ron::ser::to_string(&clipboard_data).expect("expect serialize to work..."))
        .unwrap();
}

pub fn paste_from_clipboard(state: &mut SoundGraphEditorState, cursor_pos: Vec2) {
    let mut clipboard = clippers::Clipboard::get();
    let mut data: Option<ClipboardData> = None;
    match clipboard.read() {
        Some(clippers::ClipperData::Text(text)) => match ron::de::from_str(text.as_str()) {
            Ok(x) => {
                data = Some(x);
            }
            Err(_) => {}
        },

        Some(clippers::ClipperData::Image(_)) => {}

        Some(_) => {}

        None => {}
    }

    let data = match data {
        Some(x) => x,
        None => {
            return;
        }
    };

    let mut ids = vec![];

    for (node, node_pos) in data.nodes.clone() {
        let mut _id = Default::default();
        state.graph.add_node(
            node.user_data.name.clone(),
            node.user_data.clone(),
            |_graph, id| {
                state.node_order.push(id);
                state.node_positions.insert(
                    id,
                    node_pos.clone() + cursor_pos + Vec2 { x: 0.0, y: 1000.0 },
                );
                _id = id
            },
        );
        ids.push(_id);
        for input_id in node.inputs.clone() {
            let bruh = &data.input_params[&input_id.1];
            state.graph.add_input_param(
                _id,
                input_id.0,
                bruh.typ,
                bruh.value.clone(),
                bruh.kind,
                true,
            );
        }
        for output_id in node.outputs.clone() {
            let bruh = &data.output_params[&output_id.1];
            state.graph.add_output_param(_id, output_id.0, bruh.typ);
        }
    }

    let mut input_id_map = HashMap::new();
    let mut output_id_map = HashMap::new();

    for ((node, _node_pos), node_id) in data.nodes.iter().zip(ids.clone()) {
        for (name, id) in state.graph.nodes[node_id].inputs.iter() {
            for (from_name, from_id) in node.inputs.iter() {
                if from_name == name {
                    input_id_map.insert(*from_id, *id);
                }
            }
        }
        for (name, id) in state.graph.nodes[node_id].outputs.iter() {
            for (from_name, from_id) in node.outputs.iter() {
                if from_name == name {
                    output_id_map.insert(*from_id, *id);
                }
            }
        }
    }

    for (input_id, output_id) in &data.connections {
        if input_id_map.contains_key(input_id) && output_id_map.contains_key(output_id) {
            state
                .graph
                .add_connection(output_id_map[output_id], input_id_map[input_id]);
        }
    }

    let _ = ids.iter().map(|x| state.selected_nodes.push(*x));
}
