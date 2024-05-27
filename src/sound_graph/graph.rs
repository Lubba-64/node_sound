use super::graph_types::InputValueConfig;
use super::save_management::{
    convert_option_pathbuf, get_current_exe_dir, get_current_working_settings, open_project_file,
    save_current_working_settings, save_project_file, save_project_file_as,
    set_output_sound_destination, ProjectFile, WorkingFileSettings,
};
use super::DEFAULT_SAMPLE_RATE;
use crate::nodes::{get_nodes, NodeDefinitions, SoundNode, SoundNodeProps};
use crate::sound_graph::graph_types::{DataType, ValueType};
use crate::sound_graph::save_management::get_project_file;
use crate::sound_queue;
use crate::sounds::AsGenericSource;
use eframe::egui::{self, DragValue, TextStyle};
use egui_node_graph_2::*;
use rfd::FileDialog;
use rodio::source::Zero;
use rodio::Source;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::Path;
use std::{borrow::Cow, collections::HashMap, time::Duration};

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeData {
    pub name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MyResponse {
    SetPlayingNode(NodeId),
    SetRecordingNode(NodeId),
    ClearNodeInteractions,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SoundGraphState {
    pub playing_node: Option<NodeId>,
    pub recording_node: Option<NodeId>,
    pub active_modified: bool,
    pub sound_result_evaluated: bool,
    pub recording_length: usize,
}

impl DataTypeTrait<SoundGraphState> for DataType {
    fn data_type_color(&self, _user_state: &mut SoundGraphState) -> egui::Color32 {
        match self {
            DataType::Duration => egui::Color32::from_rgb(38, 109, 211),
            DataType::Float => egui::Color32::from_rgb(238, 207, 109),
            DataType::AudioSource => egui::Color32::from_rgb(100, 150, 100),
            DataType::File => egui::Color32::from_rgb(100, 100, 150),
            DataType::None => egui::Color32::from_rgb(100, 100, 100),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            DataType::Duration => Cow::Borrowed("Duration"),
            DataType::Float => Cow::Borrowed("Float"),
            DataType::AudioSource => Cow::Borrowed("AudioSource"),
            DataType::File => Cow::Borrowed("File"),
            DataType::None => Cow::Borrowed("None"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeDefinitionUi(pub SoundNode);
impl NodeTemplateTrait for NodeDefinitionUi {
    type NodeData = NodeData;
    type DataType = DataType;
    type ValueType = ValueType;
    type UserState = SoundGraphState;
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
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        for input in self.0.inputs.iter() {
            graph.add_input_param(
                node_id,
                input.0.clone(),
                input.1.data_type,
                match &input.1.value {
                    InputValueConfig::AudioSource {} => ValueType::AudioSource { value: 0 },
                    InputValueConfig::Float { value } => ValueType::Float { value: *value },
                    InputValueConfig::Duration { value } => ValueType::Duration {
                        value: Duration::from_secs_f32(*value),
                    },
                    InputValueConfig::File { value } => ValueType::File {
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
    type Response = MyResponse;
    type UserState = SoundGraphState;
    type NodeData = NodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
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
            ValueType::File { value } => {
                let y = &value.clone();
                let file_name = match y {
                    Some(x) => std::path::Path::new(x)
                        .file_name()
                        .unwrap_or(OsStr::new(""))
                        .to_str()
                        .unwrap_or(""),
                    None => "",
                };
                if ui.button(format!("{}...", file_name)).clicked() {
                    let file = match convert_option_pathbuf(
                        FileDialog::new()
                            .add_filter("sound", &["ogg", "mp3", "wav"])
                            .set_directory("./")
                            .pick_file(),
                    ) {
                        Ok(x) => Some(x),
                        Err(_) => None,
                    };
                    *value = file
                }
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
        graph: &Graph<NodeData, DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<MyResponse, NodeData>>
    where
        MyResponse: UserResponseTrait,
    {
        let mut responses = vec![];
        let is_playing = user_state
            .playing_node
            .map(|id| id == node_id)
            .unwrap_or(false);
        if !is_playing {
            if ui.button("▶ Play").clicked() {
                responses.push(NodeResponse::User(MyResponse::SetPlayingNode(node_id)));
                user_state.active_modified = true;
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("⏹ Stop").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(MyResponse::ClearNodeInteractions));
                user_state.active_modified = true;
            }
        }
        let is_recording = user_state
            .recording_node
            .map(|id| id == node_id)
            .unwrap_or(false);
        if !is_recording {
            if ui.button("⬤ Record").clicked() {
                responses.push(NodeResponse::User(MyResponse::SetRecordingNode(node_id)));
                user_state.active_modified = true;
            }
            ui.label("Recording Duration");
            ui.add(DragValue::new(&mut user_state.recording_length));
        } else {
            let button =
                egui::Button::new(egui::RichText::new("Recording...").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(MyResponse::ClearNodeInteractions));
                user_state.active_modified = true;
            }
        }

        responses
    }
}

type MyGraph = Graph<NodeData, DataType, ValueType>;

type SoundGraphEditorState =
    GraphEditorState<NodeData, DataType, ValueType, NodeDefinitionUi, SoundGraphState>;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SoundNodeGraphSavedState {
    pub user_state: SoundGraphState,
    pub editor_state: SoundGraphEditorState,
}

pub struct SoundNodeGraph {
    pub settings_state: WorkingFileSettings,
    pub state: SoundNodeGraphSavedState,
    pub node_definitions: NodeDefinitions,
    pub stream: (OutputStream, OutputStreamHandle),
    pub sink: Sink,
    pub exe_dir: String,
    settings_path: String,
}

impl SoundNodeGraph {
    pub fn new() -> Self {
        let (stream, stream_handle) =
            OutputStream::try_default().expect("could not initialize audio subsystem");
        let exe_dir = get_current_exe_dir().expect("could not get app directory");
        let settings_path = std::path::Path::new(&exe_dir).join("node_settings.ron");
        let settings_state = match get_current_working_settings(&settings_path.to_str().unwrap()) {
            Err(_) => WorkingFileSettings::default(),
            Ok(x) => x,
        };
        Self {
            settings_path: settings_path.to_str().unwrap().to_string(),
            exe_dir: exe_dir,
            state: match &settings_state.latest_saved_file {
                None => SoundNodeGraphSavedState::default(),
                Some(x) => match get_project_file(&x) {
                    Ok(y) => y.graph_state,
                    Err(_) => SoundNodeGraphSavedState::default(),
                },
            },
            settings_state: settings_state,
            node_definitions: get_nodes(),
            sink: Sink::try_new(&stream_handle).expect("could not create audio sink"),
            stream: (stream, stream_handle),
        }
    }

    fn save_project_file(&self, path: &str) {
        let _ = save_project_file(
            ProjectFile {
                graph_state: self.state.clone(),
            },
            &path,
        );
    }

    fn save_project_settings(&mut self, new_file: String) {
        self.settings_state.latest_saved_file = Some(new_file);
        let _ = save_current_working_settings(&self.settings_path, self.settings_state.clone());
    }

    fn save_project_settings_as(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.save_project_settings(save_project_file_as(ProjectFile {
            graph_state: self.state.clone(),
        })?);
        Ok(())
    }

    fn combobox(&mut self, ui: &mut egui::Ui) {
        let combobox = egui::ComboBox::from_label("").selected_text("File");
        let _ = combobox.show_ui(ui, |ui| -> Result<(), Box<dyn std::error::Error>> {
            if ui.add(egui::Button::new("New Project")).clicked() {
                self.settings_state.latest_saved_file = None;
                self.state = SoundNodeGraphSavedState::default();
            }
            if ui.add(egui::Button::new("Save")).clicked() {
                match &self.settings_state.latest_saved_file {
                    Some(x) => self.save_project_file(x),
                    None => self.save_project_settings_as()?,
                }
            }
            if ui.add(egui::Button::new("Save As")).clicked() {
                self.save_project_settings_as()?;
            }
            if ui.add(egui::Button::new("Open")).clicked() {
                let file = open_project_file()?;
                self.state = file.1.graph_state;
                self.save_project_settings(file.0);
            }
            Ok(())
        });
    }
}

impl eframe::App for SoundNodeGraph {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                self.combobox(ui);
                ui.add(egui::Label::new(
                    match &self.settings_state.latest_saved_file {
                        Some(x) => Path::new(x)
                            .file_name()
                            .unwrap_or(&OsStr::new(""))
                            .to_str()
                            .unwrap_or(""),
                        None => "<new project>",
                    },
                ));
            });
        });
        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state.editor_state.draw_graph_editor(
                    ui,
                    NodeDefinitionsUi(&self.node_definitions),
                    &mut self.state.user_state,
                    Vec::default(),
                )
            })
            .inner;

        for node_response in graph_response.node_responses {
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    MyResponse::SetPlayingNode(node) => {
                        self.state.user_state.playing_node = Some(node);
                        self.state.user_state.recording_node = None;
                    }
                    MyResponse::SetRecordingNode(node) => {
                        self.state.user_state.recording_node = Some(node);
                        self.state.user_state.playing_node = None;
                    }
                    MyResponse::ClearNodeInteractions => {
                        self.state.user_state.playing_node = None;
                        self.state.user_state.recording_node = None;
                    }
                }
            }
        }

        let mut sound_result = None;
        let mut file_path = None;

        if self.state.user_state.active_modified {
            if let Some(node) = self.state.user_state.playing_node {
                if self.state.editor_state.graph.nodes.contains_key(node) {
                    let text;

                    match evaluate_node(
                        &self.state.editor_state.graph,
                        node,
                        &mut HashMap::new(),
                        &self.node_definitions,
                    ) {
                        Ok(value) => {
                            let sound = value.try_to_source().expect("expected valid audio source");
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
                    self.state.user_state.playing_node = None;
                }
            }

            if let Some(node) = self.state.user_state.recording_node {
                if self.state.editor_state.graph.nodes.contains_key(node) {
                    let text;

                    match evaluate_node(
                        &self.state.editor_state.graph,
                        node,
                        &mut HashMap::new(),
                        &self.node_definitions,
                    ) {
                        Ok(value) => match set_output_sound_destination() {
                            Ok(_file_path) => {
                                let sound =
                                    value.try_to_source().expect("expected valid audio source");
                                sound_result = Some(sound.clone());
                                file_path = Some(_file_path);
                                text = "Recording Anonymous audio source.";
                            }
                            Err(_err) => {
                                sound_result = None;
                                text = "An error occured trying to record the audio source.";
                            }
                        },
                        Err(_err) => {
                            sound_result = None;
                            text = "An error occured trying to record the audio source.";
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
                    self.state.user_state.playing_node = None;
                }
            }
        }
        match sound_result {
            Some(x) => {
                if self.state.user_state.playing_node.is_some() {
                    if self.state.user_state.active_modified {
                        self.sink.append(match sound_queue::clone_sound(x) {
                            Err(_) => Zero::new(1, DEFAULT_SAMPLE_RATE).as_generic(None),
                            Ok(x) => x,
                        });
                        self.sink.play();
                        self.sink.set_volume(1.0);
                        self.state.user_state.active_modified = false;
                    }
                } else if self.state.user_state.recording_node.is_some() {
                    if self.state.user_state.active_modified {
                        let file_path = file_path.unwrap();
                        let source = match sound_queue::clone_sound(x) {
                            Err(_) => Zero::new(1, DEFAULT_SAMPLE_RATE).as_generic(None),
                            Ok(x) => x,
                        };
                        let spec = hound::WavSpec {
                            channels: source.channels(),
                            sample_rate: DEFAULT_SAMPLE_RATE,
                            bits_per_sample: 32,
                            sample_format: hound::SampleFormat::Float,
                        };
                        let mut writer = hound::WavWriter::create(file_path, spec).unwrap();
                        for (idx, sample) in source.enumerate() {
                            if idx / DEFAULT_SAMPLE_RATE as usize
                                > self.state.user_state.recording_length
                            {
                                break;
                            }
                            let _ = writer.write_sample(sample).unwrap();
                        }
                        let _ = writer.finalize().unwrap();
                        self.state.user_state.recording_node = None;
                    }
                }
            }
            None => {
                if self.state.user_state.active_modified {
                    self.sink.clear();
                    sound_queue::clear();
                    self.state.user_state.active_modified = false;
                }
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
) -> Result<ValueType, Box<dyn std::error::Error>> {
    let node = match all_nodes.0.get(&graph[node_id].user_data.name) {
        Some(x) => x,
        None => panic!("Node deref failed"),
    };

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
        node.0
            .inputs
            .iter()
            .map(|(name, _input)| (closure)(name.to_string())),
    );

    let res = (node.1)(SoundNodeProps {
        inputs: input_to_name,
    })?;

    for (name, value) in &res {
        match populate_output(graph, outputs_cache, node_id, &name, value.clone()) {
            Ok(_x) => (),
            Err(_x) => panic!("Output failed to populate"),
        }
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
