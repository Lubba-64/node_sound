use super::copy_paste_del_helpers::{
    copy_to_clipboard, delete_selected_nodes, paste_from_clipboard,
};
use super::float_selector;
use super::graph_types::InputValueConfig;
use super::wave_table_graph::wave_table_graph;
use crate::constants::DEFAULT_SAMPLE_RATE;
use crate::nodes::{get_nodes, NodeDefinitions, SoundNode, SoundNodeProps};
use crate::sound_graph::graph_types::{DataType, ValueType};
use crate::sound_map;
use eframe::egui::Pos2;
use eframe::egui::{self, DragValue, Vec2};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
pub use egui_node_graph_2::*;
use futures::executor;
pub use rodio::source::Zero;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::{borrow::Cow, collections::HashMap, time::Duration};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NodeData {
    pub name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ActiveNodeState {
    PlayingNode(NodeId),
    RecordingNode(NodeId),
    #[default]
    NoNode,
}

impl ActiveNodeState {
    pub fn is_playing(&self) -> bool {
        match self {
            ActiveNodeState::PlayingNode(_) => return true,
            _ => return false,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct SoundGraphUserState {
    pub active_node: ActiveNodeState,
    pub active_modified: bool,
    pub sound_result_evaluated: bool,
    pub recording_length: usize,
    pub is_saved: bool,
    pub vst_output_node_id: Option<NodeId>,
    #[serde(skip)]
    pub is_done_showing_recording_dialogue: bool,
    pub wave_shaper_graph_id: Option<usize>,
}

impl Clone for SoundGraphUserState {
    fn clone(&self) -> Self {
        Self {
            active_node: self.active_node,
            active_modified: self.active_modified,
            sound_result_evaluated: self.sound_result_evaluated,
            recording_length: self.recording_length,
            is_saved: self.is_saved,
            vst_output_node_id: None,
            is_done_showing_recording_dialogue: self.is_done_showing_recording_dialogue,
            wave_shaper_graph_id: self.wave_shaper_graph_id,
        }
    }
}

impl DataTypeTrait<SoundGraphUserState> for DataType {
    fn data_type_color(&self, _user_state: &mut SoundGraphUserState) -> egui::Color32 {
        match self {
            DataType::Duration => egui::Color32::from_rgb(38, 109, 211),
            DataType::Float => egui::Color32::from_rgb(238, 207, 109),
            DataType::AudioSource => egui::Color32::from_rgb(100, 150, 100),
            DataType::AudioFile => egui::Color32::from_rgb(100, 100, 150),
            DataType::None => egui::Color32::from_rgb(100, 100, 100),
            DataType::MidiFile => egui::Color32::from_rgb(150, 100, 100),
            DataType::Graph => egui::Color32::from_rgb(150, 100, 100),
            DataType::Code => egui::Color32::from_rgb(150, 100, 100),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            DataType::Duration => Cow::Borrowed("Duration"),
            DataType::Float => Cow::Borrowed("Float"),
            DataType::AudioSource => Cow::Borrowed("AudioSource"),
            DataType::AudioFile => Cow::Borrowed("File"),
            DataType::None => Cow::Borrowed("None"),
            DataType::MidiFile => Cow::Borrowed("Midi"),
            DataType::Graph => Cow::Borrowed("Graph"),
            DataType::Code => Cow::Borrowed("Code"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeDefinitionUi(pub SoundNode);

impl NodeTemplateTrait for NodeDefinitionUi {
    type NodeData = NodeData;
    type DataType = DataType;
    type ValueType = ValueType;
    type UserState = SoundGraphUserState;
    type CategoryType = ();

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        Cow::Owned(self.0.name.clone())
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
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
        user_state.is_saved = false;
        for input in self.0.inputs.iter() {
            graph.add_input_param(
                node_id,
                input.0.clone(),
                input.1.data_type,
                match &input.1.value {
                    InputValueConfig::Code { value } => ValueType::Code {
                        value: value.clone(),
                    },
                    InputValueConfig::AudioSource {} => ValueType::AudioSource { value: 0 },
                    InputValueConfig::Float { value, min, max } => ValueType::Float {
                        value: *value,
                        min: *min,
                        max: *max,
                        note: super::note::NoteValue::default(),
                    },
                    InputValueConfig::Duration { value } => ValueType::Duration {
                        value: Duration::from_secs_f32(*value),
                    },
                    InputValueConfig::AudioFile {} => ValueType::AudioFile { value: None },
                    InputValueConfig::MidiFile {} => ValueType::MidiFile { value: None },
                    InputValueConfig::Graph { value } => {
                        user_state.wave_shaper_graph_id =
                            Some(user_state.wave_shaper_graph_id.unwrap_or(0) + 1);
                        ValueType::Graph {
                            value: Some(value.clone()),
                            id: user_state.wave_shaper_graph_id.unwrap(),
                        }
                    }
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

pub struct NodeDefinitionsUi<'a>(&'a NodeDefinitions, &'a SoundNodeGraphState);
impl<'a> NodeTemplateIter for NodeDefinitionsUi<'a> {
    type Item = NodeDefinitionUi;

    fn all_kinds(&self) -> Vec<Self::Item> {
        let mut contains: HashSet<String> = HashSet::new();
        for node_id in self.1.editor_state.graph.iter_nodes() {
            contains.insert(
                self.1.editor_state.graph.nodes[node_id]
                    .user_data
                    .name
                    .to_string(),
            );
        }
        self.0
             .0
            .values()
            .cloned()
            .map(|x| x.0)
            .map(NodeDefinitionUi)
            .filter(|x| {
                for node in vec!["Output", "DAW Automation"].iter() {
                    if contains.contains(&node.to_string())
                        && x.0.name.to_string() == node.to_string()
                    {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}

impl WidgetValueTrait for ValueType {
    type Response = ActiveNodeState;
    type UserState = SoundGraphUserState;
    type NodeData = NodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<ActiveNodeState> {
        match self {
            ValueType::Code { value } => egui::Resize::default().show(ui, |ui| {
                CodeEditor::default()
                    .id_source("code editor")
                    .with_rows(24)
                    .with_fontsize(14.0)
                    .with_theme(ColorTheme::GRUVBOX)
                    .with_syntax(Syntax::rust())
                    .with_numlines(true)
                    .show(ui, value);
            }),
            ValueType::Graph { value, id } => wave_table_graph(value, ui, *id),
            ValueType::Float {
                value,
                min,
                max,
                note,
            } => float_selector::float_selector(value, min, max, note, ui, param_name),
            ValueType::Duration { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    let mut secs_f32 = value.as_secs_f32();
                    ui.add(DragValue::new(&mut secs_f32).speed(0.01));
                    *value = Duration::from_secs_f32(secs_f32.max(0.0));
                });
            }
            ValueType::AudioSource { value: _ } => {
                ui.label(param_name);
            }
            ValueType::None => {
                ui.label("None");
            }
            ValueType::AudioFile { value } => {
                let y = &value.clone();
                let file_name = match y {
                    Some(x) => std::path::Path::new(&x.0)
                        .file_name()
                        .unwrap_or(OsStr::new(""))
                        .to_str()
                        .unwrap_or(""),
                    None => "",
                };
                if ui.button(format!("{}...", file_name)).clicked() {
                    // TODO: FILE GET.
                }
            }
            ValueType::MidiFile { value } => {
                let y = &value.clone();
                let file_name = match y {
                    Some(x) => std::path::Path::new(&x.0)
                        .file_name()
                        .unwrap_or(OsStr::new(""))
                        .to_str()
                        .unwrap_or(""),
                    None => "",
                };
                if ui.button(format!("{}...", file_name)).clicked() {
                    // TODO: FILE GET.
                }
            }
        }
        Vec::new()
    }
}

impl UserResponseTrait for ActiveNodeState {}
impl NodeDataTrait for NodeData {
    type Response = ActiveNodeState;
    type UserState = SoundGraphUserState;
    type DataType = DataType;
    type ValueType = ValueType;

    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<NodeData, DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<ActiveNodeState, NodeData>>
    where
        ActiveNodeState: UserResponseTrait,
    {
        let mut responses = vec![];
        let is_playing: bool = match user_state.active_node {
            ActiveNodeState::PlayingNode(x) => x == node_id,
            _ => false,
        };
        if !is_playing {
            if ui.button("▶ Play").clicked() {
                if user_state.active_node == ActiveNodeState::NoNode {
                    responses.push(NodeResponse::User(ActiveNodeState::PlayingNode(node_id)));
                    user_state.active_modified = true;
                }
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("⏹ Stop").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(ActiveNodeState::NoNode));
                user_state.active_modified = true;
            }
        }

        responses
    }
}

type MyGraph = Graph<NodeData, DataType, ValueType>;

pub type SoundGraphEditorState =
    GraphEditorState<NodeData, DataType, ValueType, NodeDefinitionUi, SoundGraphUserState>;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SoundNodeGraphState {
    pub user_state: SoundGraphUserState,
    pub editor_state: SoundGraphEditorState,
}

#[derive(Default)]
pub struct UnserializeableGraphState {
    pub node_definitions: Option<NodeDefinitions>,
    pub _stream: Option<(OutputStream, OutputStreamHandle)>,
    pub sink: Option<Sink>,
}

pub fn get_unserializeable_graph_state() -> UnserializeableGraphState {
    return UnserializeableGraphState {
        node_definitions: Some(get_nodes()),
        sink: None,
        _stream: None,
    };
}

#[derive(Serialize, Deserialize, Default)]
pub struct SoundNodeGraph {
    pub state: SoundNodeGraphState,
    #[serde(skip)]
    pub _unserializeable_state: UnserializeableGraphState,
}

unsafe impl Send for SoundNodeGraph {}
unsafe impl Sync for SoundNodeGraph {}

fn new_sound_node_graph() -> SoundNodeGraph {
    SoundNodeGraph {
        state: SoundNodeGraphState {
            editor_state: SoundGraphEditorState::default(),
            user_state: SoundGraphUserState {
                ..Default::default()
            },
        },
        _unserializeable_state: get_unserializeable_graph_state(),
    }
}

impl SoundNodeGraph {
    pub fn new_vst_synth() -> Self {
        new_sound_node_graph()
    }

    pub fn new_vst_effect() -> Self {
        new_sound_node_graph()
    }

    fn update_output_node(&mut self) {
        let mut found = false;
        for node in self.state.editor_state.graph.iter_nodes() {
            let found_match =
                self.state.editor_state.graph.nodes.get(node).unwrap().label == "Output";
            if found_match {
                found = true;
                self.state.user_state.vst_output_node_id = Some(node)
            }
        }
        if !found {
            self.state.user_state.vst_output_node_id = None;
        }
    }

    pub fn update_root(&mut self, ctx: &egui::Context) {
        self.update_output_node();
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);
                ui.add(egui::Label::new(env!("CARGO_PKG_VERSION")));
                ui.add(egui::Label::new("|"));
                if ui.add(egui::Button::new("copy")).clicked() {
                    copy_to_clipboard(&mut self.state.editor_state);
                }
                if ui.add(egui::Button::new("paste")).clicked() {
                    let input =
                        ui.input(|i| i.pointer.latest_pos().unwrap_or(Pos2 { x: 0.0, y: 0.0 }));
                    let input_vec2 = Vec2 {
                        x: input.x,
                        y: input.y,
                    };
                    executor::block_on(paste_from_clipboard(
                        &mut self.state.editor_state,
                        input_vec2,
                    ));
                }
                if ui.add(egui::Button::new("delete selected")).clicked() {
                    delete_selected_nodes(&mut self.state.editor_state)
                }
            });
        });

        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state.editor_state.draw_graph_editor(
                    ui,
                    NodeDefinitionsUi(
                        &self
                            ._unserializeable_state
                            .node_definitions
                            .as_ref()
                            .unwrap(),
                        &self.state.clone(),
                    ),
                    &mut self.state.user_state,
                    Vec::default(),
                )
            })
            .inner;

        for node_response in graph_response.node_responses {
            if let NodeResponse::User(user_event) = node_response {
                self.state.user_state.active_node = user_event;
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn open_url(url: &str) {
    let window = web_sys::window().expect("failed to retrieve window");
    let _ = window.open_with_url(url);
}

impl eframe::App for SoundNodeGraph {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_root(ctx)
    }
}

type OutputsCache<'a> = HashMap<OutputId, ValueType>;

/// Recursively evaluates all dependencies of this node, then evaluates the node itself.
pub fn evaluate_node<'a>(
    graph: &MyGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
    all_nodes: &NodeDefinitions,
) -> Result<ValueType, Box<dyn std::error::Error>> {
    let node = match all_nodes.0.get(&graph[node_id].user_data.name) {
        Some(x) => x,
        None => panic!("Node deref failed"),
    };

    let mut closure = |name: String| {
        (
            name.clone(),
            evaluate_input(graph, node_id, name.as_str(), outputs_cache, all_nodes),
        )
    };
    let input_to_name = HashMap::from_iter(
        node.0
            .inputs
            .iter()
            .map(|(name, _input)| (closure)(name.to_string())),
    );

    let res = (node.1)(SoundNodeProps {
        inputs: input_to_name,
    })?;

    for (name, value) in &res {
        populate_output(graph, outputs_cache, node_id, &name, value.clone());
    }

    match res.get("out") {
        Some(x) => Ok(x.clone()),
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Node had no output",
        ))),
    }
}

fn populate_output<'a>(
    graph: &'a MyGraph,
    outputs_cache: &'a mut OutputsCache,
    node_id: NodeId,
    param_name: &'a str,
    value: ValueType,
) -> ValueType {
    let output_id = match graph[node_id].get_output(param_name) {
        Ok(x) => x,
        Err(_x) => panic!("EGUI node graph error"),
    };
    outputs_cache.insert(output_id, value.clone());
    value
}

fn evaluate_input<'a>(
    graph: &'a MyGraph,
    node_id: NodeId,
    param_name: &'a str,
    outputs_cache: &'a mut OutputsCache,
    all_nodes: &'a NodeDefinitions,
) -> ValueType {
    let input_id = match graph[node_id].get_input(param_name) {
        Ok(x) => x,
        Err(_x) => panic!("EGUI node graph error"),
    };

    if let Some(other_output_id) = graph.connection(input_id) {
        if let Some(other_value) = outputs_cache.get(&other_output_id) {
            other_value.clone()
        } else {
            match evaluate_node(graph, graph[other_output_id].node, outputs_cache, all_nodes) {
                Ok(x) => x,
                Err(_x) => ValueType::AudioSource {
                    value: sound_map::push_sound(Box::new(Zero::new(1, DEFAULT_SAMPLE_RATE))),
                },
            };
            outputs_cache
                .get(&other_output_id)
                .unwrap_or(&ValueType::AudioSource {
                    value: sound_map::push_sound(Box::new(Zero::new(1, DEFAULT_SAMPLE_RATE))),
                })
                .clone()
        }
    } else {
        graph[input_id].value.clone()
    }
}
