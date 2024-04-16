use super::graph_types::InputValueConfig;
use crate::nodes::{get_nodes, NodeDefinitions, SoundNode, SoundNodeProps};
use crate::sound_graph::graph_types::{DataType, ValueType};
use crate::sound_queue;
use eframe::egui::{self, DragValue, TextStyle};
use egui_node_graph_2::*;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::any::Any;
use std::{borrow::Cow, collections::HashMap, time::Duration};

#[derive(Clone)]
pub struct NodeData {
    pub name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MyResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
}

#[derive(Default)]
pub struct SoundGraphState {
    pub active_node: Option<NodeId>,
    pub active_modified: bool,
}

impl DataTypeTrait<SoundGraphState> for DataType {
    fn data_type_color(&self, _user_state: &mut SoundGraphState) -> egui::Color32 {
        match self {
            DataType::Duration => egui::Color32::from_rgb(38, 109, 211),
            DataType::Float => egui::Color32::from_rgb(238, 207, 109),
            DataType::AudioSource => egui::Color32::from_rgb(100, 100, 100),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            DataType::Duration => Cow::Borrowed("Duration"),
            DataType::Float => Cow::Borrowed("Float"),
            DataType::AudioSource => Cow::Borrowed("AudioSource"),
        }
    }
}

#[derive(Clone)]
pub struct NodeDefinitionUi(pub SoundNode);
impl NodeTemplateTrait for NodeDefinitionUi {
    type NodeData = NodeData;
    type DataType = DataType;
    type ValueType = ValueType;
    type UserState = SoundGraphState;
    type CategoryType = ();

    fn node_finder_label(&self, user_state: &mut Self::UserState) -> Cow<'_, str> {
        Cow::Owned(self.0.name.clone())
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, user_state: &mut Self::UserState) -> Self::NodeData {
        NodeData {
            name: self.0.name.clone(),
        }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        for input in self.0.inputs.iter() {
            graph.add_input_param(
                node_id,
                input.0.clone(),
                input.1.data_type,
                match input.1.value {
                    InputValueConfig::AudioSource {} => ValueType::AudioSource { value: 0 },
                    InputValueConfig::Float { value } => ValueType::Float { value },
                    InputValueConfig::Duration { value } => ValueType::Duration {
                        value: Duration::from_secs_f32(value),
                    },
                },
                input.1.kind,
                true,
            );
        }
        for output in self.0.outputs.iter() {
            graph.add_output_param(node_id, output.0.clone(), output.1.data_type);
        }
    }
}

pub struct NodeDefinitionsUi<'a>(&'a NodeDefinitions);
impl<'a> NodeTemplateIter for NodeDefinitionsUi<'a> {
    type Item = NodeDefinitionUi;

    fn all_kinds(&self) -> Vec<Self::Item> {
        self.0 .0.values().cloned().map(NodeDefinitionUi).collect()
    }
}

impl WidgetValueTrait for ValueType {
    type Response = MyResponse;
    type UserState = SoundGraphState;
    type NodeData = NodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        node_id: NodeId,
        ui: &mut egui::Ui,
        user_state: &mut Self::UserState,
        node_data: &Self::NodeData,
    ) -> Vec<MyResponse> {
        match self {
            ValueType::Float { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value));
                });
            }
            ValueType::Duration { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    let mut secs_f32 = value.as_secs_f32();
                    ui.add(DragValue::new(&mut secs_f32));
                    *value = Duration::from_secs_f32(secs_f32.max(0.0));
                });
            }
            ValueType::AudioSource { value: _ } => {
                ui.label(param_name);
            }
            ValueType::None => {
                ui.label("None");
            }
        }
        Vec::new()
    }
}

impl UserResponseTrait for MyResponse {}
impl NodeDataTrait for NodeData {
    type Response = MyResponse;
    type UserState = SoundGraphState;
    type DataType = DataType;
    type ValueType = ValueType;

    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<NodeData, DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<MyResponse, NodeData>>
    where
        MyResponse: UserResponseTrait,
    {
        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);
        if !is_active {
            if ui.button("▶ Play").clicked() {
                responses.push(NodeResponse::User(MyResponse::SetActiveNode(node_id)));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("⏹ Stop").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(MyResponse::ClearActiveNode));
            }
        }
        responses
    }
}

type MyGraph = Graph<NodeData, DataType, ValueType>;

type MyEditorState =
    GraphEditorState<NodeData, DataType, ValueType, NodeDefinitionUi, SoundGraphState>;

pub struct NodeGraphExample {
    pub user_state: SoundGraphState,
    pub state: MyEditorState,
    pub node_definitions: NodeDefinitions,
    pub stream: (OutputStream, OutputStreamHandle),
    pub sink: Sink,
}

impl NodeGraphExample {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            state: MyEditorState::new(1.0),
            node_definitions: get_nodes(),
            sink: Sink::try_new(&stream_handle).unwrap(),
            stream: (stream, stream_handle),
            user_state: SoundGraphState::default(),
        }
    }
}

impl eframe::App for NodeGraphExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });
        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state.draw_graph_editor(
                    ui,
                    NodeDefinitionsUi(&self.node_definitions),
                    &mut self.user_state,
                    Vec::default(),
                )
            })
            .inner;

        for node_response in graph_response.node_responses {
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    MyResponse::SetActiveNode(node) => self.user_state.active_node = Some(node),
                    MyResponse::ClearActiveNode => self.user_state.active_node = None,
                }
            }
        }

        let mut sound_result = None;

        if let Some(node) = self.user_state.active_node {
            if self.state.graph.nodes.contains_key(node) {
                let text;

                match evaluate_node(
                    &self.state.graph,
                    node,
                    &mut HashMap::new(),
                    &self.node_definitions,
                ) {
                    Ok(value) => {
                        let sound = value.try_to_source().unwrap();
                        sound_result = Some(sound.clone());
                        text = "Playing Anonymous audio source.";
                    }
                    Err(_err) => {
                        sound_result = None;
                        text = "An error occured trying to play the audio source.";
                    }
                };

                ctx.debug_painter().text(
                    egui::pos2(10.0, 35.0),
                    egui::Align2::LEFT_TOP,
                    text,
                    TextStyle::Button.resolve(&ctx.style()),
                    egui::Color32::WHITE,
                );
            } else {
                self.user_state.active_node = None;
            }
        }

        match sound_result {
            Some(x) => {
                if self.user_state.active_modified {
                    self.sink.append(sound_queue::clone_sound(x));
                    self.sink.play();
                    self.sink.set_volume(1.0);
                    self.user_state.active_modified = false;
                }
            }
            None => {
                self.sink.clear();
            }
        }
    }
}

type OutputsCache<'a> = HashMap<OutputId, ValueType>;

/// Recursively evaluates all dependencies of this node, then evaluates the node itself.
pub fn evaluate_node<'a>(
    graph: &MyGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
    all_nodes: &NodeDefinitions,
) -> Result<ValueType, &'a str> {
    let node = match all_nodes.0.get(&graph[node_id].user_data.name) {
        Some(x) => x,
        None => panic!("Node deref failed"),
    };

    // graph.connections.values().into_iter().filter(|x| {x.})

    let mut closure = |name: String| {
        (
            name.clone(),
            match evaluate_input(graph, node_id, name.as_str(), outputs_cache, all_nodes) {
                Ok(x) => x,
                Err(_x) => panic!("Input resolution failed"),
            },
        )
    };
    let input_to_name = HashMap::from_iter(
        node.inputs
            .iter()
            .map(|(name, _input)| (closure)(name.to_string())),
    );

    let res = (node.operation)(SoundNodeProps {
        inputs: input_to_name,
    });

    for (name, value) in res.iter() {
        match populate_output(graph, outputs_cache, node_id, name, value.clone()) {
            Ok(_x) => (),
            Err(_x) => panic!("Output failed to populate"),
        }
    }

    match res.get("out") {
        Some(x) => Ok(x.clone()),
        None => Err("Node had no output"),
    }
}

fn populate_output<'a>(
    graph: &'a MyGraph,
    outputs_cache: &'a mut OutputsCache,
    node_id: NodeId,
    param_name: &'a str,
    value: ValueType,
) -> Result<ValueType, &'a str> {
    let output_id = match graph[node_id].get_output(param_name) {
        Ok(x) => x,
        Err(_x) => panic!("EGUI node graph error"),
    };
    outputs_cache.insert(output_id, value.clone());
    Ok(value)
}

fn evaluate_input<'a>(
    graph: &'a MyGraph,
    node_id: NodeId,
    param_name: &'a str,
    outputs_cache: &'a mut OutputsCache,
    all_nodes: &'a NodeDefinitions,
) -> Result<ValueType, &'a str> {
    let input_id = match graph[node_id].get_input(param_name) {
        Ok(x) => x,
        Err(_x) => panic!("EGUI node graph error"),
    };

    // The output of another node is connected.
    if let Some(other_output_id) = graph.connection(input_id) {
        // The value was already computed due to the evaluation of some other
        // node. We simply return value from the cache.
        if let Some(other_value) = outputs_cache.get(&other_output_id) {
            Ok(other_value.clone())
        }
        // This is the first time encountering this node, so we need to
        // recursively evaluate it.
        else {
            // Calling this will populate the cache
            match evaluate_node(graph, graph[other_output_id].node, outputs_cache, all_nodes) {
                Ok(x) => x,
                Err(_x) => panic!("eval failed"),
            };

            // Now that we know the value is cached, return it
            Ok(outputs_cache
                .get(&other_output_id)
                .expect("Cache should be populated")
                .clone())
        }
    }
    // No existing connection, take the inline value instead.
    else {
        Ok(graph[input_id].value.clone())
    }
}
