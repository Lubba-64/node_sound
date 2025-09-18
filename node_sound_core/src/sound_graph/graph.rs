use super::copy_paste_del_helpers::{copy, delete_nodes, paste};
use super::float_selector;
use super::graph_types::InputValueConfig;
use super::wave_table_graph::wave_table_graph;
use crate::nodes::{NodeDefinitions, SoundNode, SoundNodeProps};
use crate::sound_graph::copy_paste_del_helpers::ClipboardData;
use crate::sound_graph::graph_types::{DataType, ValueType};
use crate::sound_map::SoundQueue;
use eframe::egui::{self, DragValue, Vec2, Widget};
use eframe::egui::{Checkbox, Pos2, WidgetText};
pub use egui_node_graph_2::*;
use futures::executor;
pub use rodio::source::Zero;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::sync::{Arc, Mutex};
use std::{borrow::Cow, collections::HashMap, time::Duration};
use synthrs::midi;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
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

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct SoundGraphUserState {
    pub active_node: ActiveNodeState,
    pub active_modified: bool,
    pub sound_result_evaluated: bool,
    pub recording_length: usize,
    pub is_saved: bool,
    pub vst_output_node_id: Option<NodeId>,
    pub wave_shaper_graph_id: usize,
    #[serde(skip)]
    pub files: Arc<Mutex<FileManager>>,
    #[serde(skip)]
    ctx: egui::Context,
}

impl DataTypeTrait<SoundGraphUserState> for DataType {
    fn data_type_color(&self, _user_state: &mut SoundGraphUserState) -> egui::Color32 {
        match self {
            DataType::Duration => egui::Color32::from_rgb(38, 109, 211),
            DataType::Float => egui::Color32::from_rgb(238, 207, 109),
            DataType::AudioSource => egui::Color32::from_rgb(100, 150, 100),
            DataType::AudioFile => egui::Color32::from_rgb(100, 100, 150),
            DataType::None => egui::Color32::from_rgb(100, 100, 255),
            DataType::MidiFile => egui::Color32::from_rgb(150, 100, 255),
            DataType::Graph => egui::Color32::from_rgb(150, 100, 100),
            DataType::Bool => egui::Color32::from_rgb(150, 150, 150),
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
            DataType::Bool => Cow::Borrowed("Bool"),
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

    fn tooltip(&self) -> Option<String> {
        Some(self.0.tooltip.clone())
    }

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
                    InputValueConfig::Graph {
                        value,
                        width,
                        height,
                    } => {
                        user_state.wave_shaper_graph_id += 1;
                        ValueType::Graph {
                            value: Some(value.clone()),
                            id: user_state.wave_shaper_graph_id,
                            width: *width,
                            height: *height,
                        }
                    }
                    InputValueConfig::Bool { value } => ValueType::Bool {
                        value: value.clone(),
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
        self.0
            .0
            .values()
            .cloned()
            .map(|x| x.0)
            .map(NodeDefinitionUi)
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
        node_id: NodeId,
        ui: &mut egui::Ui,
        user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<ActiveNodeState> {
        match self {
            ValueType::Bool { value } => {
                Checkbox::new(value, WidgetText::from(param_name)).ui(ui);
            }
            ValueType::Graph {
                value,
                id,
                height,
                width,
            } => wave_table_graph(value, ui, *id, *height, *width),
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
                    ui.add(DragValue::new(&mut secs_f32).speed(0.001));
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
                ui.label(file_name);
                match user_state.files.lock() {
                    Ok(mut files) => {
                        if ui.button(format!("{}...", file_name)).clicked() {
                            files.wav_active = Some(node_id);
                        }
                        match &files.wav_file_path {
                            Some(x) => {
                                if node_id == x.1 {
                                    match fs::read(x.0.clone()) {
                                        Err(_x) => {}
                                        Ok(x2) => *value = Some((x.0.clone(), x2)),
                                    };
                                }
                            }
                            None => {}
                        };
                    }
                    Err(_) => {}
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
                match user_state.files.lock() {
                    Ok(mut files) => {
                        if ui.button(format!("{}...", file_name)).clicked() {
                            files.midi_active = Some(node_id);
                        }
                        match &files.midi_file_path {
                            Some(x) => {
                                if node_id == x.1 {
                                    match midi::read_midi_file(x.0.clone()) {
                                        Err(_x) => {}
                                        Ok(x2) => *value = Some((x.0.clone(), x2)),
                                    };
                                }
                            }
                            None => {}
                        };
                    }
                    Err(_) => {}
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
    #[serde(skip)]
    pub _unserializeable_state: UnserializeableGraphState,
}

#[derive(Default, Clone)]
pub struct FileManager {
    pub midi_active: Option<NodeId>,
    pub wav_active: Option<NodeId>,
    pub midi_file_path: Option<(String, NodeId)>,
    pub wav_file_path: Option<(String, NodeId)>,
}

#[derive(Default, Clone)]
pub struct UnserializeableGraphState {
    pub node_definitions: NodeDefinitions,
    pub is_done_showing_recording_dialogue: bool,
    pub queue: SoundQueue,
    pub automations: DAWAutomations,
    pub input: DAWInput,
}

#[derive(Default, Clone)]
pub struct DAWAutomations(pub [Arc<Mutex<f32>>; 18]);

#[derive(Default, Clone)]
pub struct DAWInput(pub Arc<Mutex<(f32, f32)>>);

#[derive(Serialize, Deserialize, Default)]
pub struct SoundNodeGraph {
    pub state: SoundNodeGraphState,
}

unsafe impl Send for SoundNodeGraph {}
unsafe impl Sync for SoundNodeGraph {}

impl SoundNodeGraph {
    pub fn new_vst_synth() -> Self {
        SoundNodeGraph::default()
    }

    pub fn new_vst_effect() -> Self {
        SoundNodeGraph::default()
    }

    pub fn new_app(cc: Option<&eframe::CreationContext<'_>>) -> Self {
        if cc.is_some() {
            if let Some(storage) = cc.unwrap().storage {
                return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            }
        }
        SoundNodeGraph::default()
    }

    fn update_output_node(&mut self) {
        let mut found = false;
        for node in self.state.editor_state.graph.iter_nodes() {
            let found_match = match self.state.editor_state.graph.nodes.get(node) {
                None => false,
                Some(x) => x.label == "Output",
            };
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
        self.state.user_state.ctx = ctx.clone();
        self.update_output_node();
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);
                ui.add(egui::Label::new(env!("CARGO_PKG_VERSION")));
                ui.add(egui::Label::new("|"));
                if ui.add(egui::Button::new("copy")).clicked() {
                    let data = copy(&mut self.state.editor_state, false);
                    let mut clipboard =
                        arboard::Clipboard::new().expect("clipboard creation failed");
                    clipboard
                        .set()
                        .text(ron::ser::to_string(&data).expect("expect serialize to work..."))
                        .expect("clipboard write failed");
                }
                if ui.add(egui::Button::new("paste")).clicked() {
                    let input =
                        ui.input(|i| i.pointer.latest_pos().unwrap_or(Pos2 { x: 0.0, y: 0.0 }));
                    let input_vec2 = Vec2 {
                        x: input.x,
                        y: input.y,
                    } + Vec2 { x: 0.0, y: 1000.0 };
                    let mut clipboard =
                        arboard::Clipboard::new().expect("clipboard creation failed");
                    let data: ClipboardData =
                        ron::de::from_str(&clipboard.get().text().expect("clipboard read failed"))
                            .expect("expect deserialize to work...");
                    executor::block_on(paste(&mut self.state.editor_state, Some(input_vec2), data));
                }
                if ui.add(egui::Button::new("delete selected")).clicked() {
                    delete_nodes(&mut self.state.editor_state, false);
                }
            });
        });

        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state.editor_state.draw_graph_editor(
                    ui,
                    NodeDefinitionsUi(&self.state._unserializeable_state.node_definitions),
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

impl eframe::App for SoundNodeGraph {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_root(ctx)
    }
}

type OutputsCache<'a> = HashMap<OutputId, ValueType>;

pub fn evaluate_node<'a>(
    graph: &MyGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
    all_nodes: &NodeDefinitions,
    state: &'a mut SoundNodeGraphState,
) -> Result<ValueType, Box<dyn std::error::Error>> {
    let node = match all_nodes.0.get(
        &match graph.nodes.get(node_id) {
            Some(x) => x,
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Node Deref Failed",
                )));
            }
        }
        .user_data
        .name,
    ) {
        Some(x) => x,
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Node Deref Failed",
            )));
        }
    };

    let mut closure = |name: String| {
        (
            name.clone(),
            evaluate_input(
                graph,
                node_id,
                name.as_str(),
                outputs_cache,
                all_nodes,
                state,
            ),
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
        state: &mut state._unserializeable_state,
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
    let output_id = match match graph.nodes.get(node_id) {
        Some(x) => x,
        None => return ValueType::AudioSource { value: 0 },
    }
    .get_output(param_name)
    {
        Ok(x) => x,
        Err(_x) => return ValueType::AudioSource { value: 0 },
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
    state: &'a mut SoundNodeGraphState,
) -> ValueType {
    let input_id = match match graph.nodes.get(node_id) {
        Some(x) => x,
        None => {
            return ValueType::AudioSource { value: 0 };
        }
    }
    .get_input(param_name)
    {
        Ok(x) => x,
        Err(_x) => {
            return ValueType::AudioSource { value: 0 };
        }
    };
    if let Some(other_output_id) = graph.connection(input_id) {
        if let Some(other_value) = outputs_cache.get(&other_output_id) {
            other_value.clone()
        } else {
            match evaluate_node(
                graph,
                graph[other_output_id].node,
                outputs_cache,
                all_nodes,
                state,
            ) {
                Ok(x) => x,
                Err(_x) => ValueType::AudioSource { value: 0 },
            };
            outputs_cache
                .get(&other_output_id)
                .unwrap_or(&ValueType::AudioSource { value: 0 })
                .clone()
        }
    } else {
        match graph.inputs.get(input_id) {
            None => return ValueType::AudioSource { value: 0 },
            Some(x) => x,
        }
        .value
        .clone()
    }
}
