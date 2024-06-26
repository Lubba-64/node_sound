use super::graph_types::InputValueConfig;
use super::save_management::{
    get_current_exe_dir, get_current_working_settings, get_input_sound, open_project_file,
    save_current_working_settings, save_project_file, save_project_file_as, write_output_sound,
    ProjectFile, WasmAsyncResolver, WorkingFileSettings,
};
use super::DEFAULT_SAMPLE_RATE;
use crate::macros::macros::crate_version;
use crate::nodes::{get_nodes, NodeDefinitions, SoundNode, SoundNodeProps};
use crate::sound_graph::graph_types::{DataType, ValueType};
use crate::sound_graph::save_management::get_project_file;
use crate::sound_map;
#[cfg(target_arch = "wasm32")]
use crate::sound_map::RefSource;
use eframe::egui::{self, DragValue, KeyboardShortcut, Modifiers, TextStyle};
use egui_node_graph_2::*;
use rodio::source::Source;
#[cfg(target_arch = "wasm32")]
use rodio::source::UniformSourceIterator;
use rodio::source::Zero;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::io::{BufWriter, Cursor};
use std::path::Path;
use std::{borrow::Cow, collections::HashMap, time::Duration};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::AudioContext;

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Default)]
pub struct UnserializeableState {
    pub save_as_wasm_future:
        Option<WasmAsyncResolver<Result<std::string::String, Box<dyn std::error::Error>>>>,
    pub open_project_file_wasm_future: Option<
        WasmAsyncResolver<Result<(std::string::String, ProjectFile), Box<(dyn std::error::Error)>>>,
    >,
    pub input_sound_wasm_future: Option<(
        WasmAsyncResolver<Result<(Vec<u8>, String), Box<dyn std::error::Error>>>,
        NodeId,
    )>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct SoundGraphState {
    pub active_node: ActiveNodeState,
    pub active_modified: bool,
    pub sound_result_evaluated: bool,
    pub recording_length: usize,
    pub is_saved: bool,
    pub is_done_showing_recording_dialogue: bool,
    #[serde(skip)]
    pub _unserializeable_state: Option<UnserializeableState>,
}

impl Clone for SoundGraphState {
    fn clone(&self) -> Self {
        Self {
            active_node: self.active_node,
            active_modified: self.active_modified,
            sound_result_evaluated: self.sound_result_evaluated,
            recording_length: self.recording_length,
            is_saved: self.is_saved,
            is_done_showing_recording_dialogue: self.is_done_showing_recording_dialogue,
            _unserializeable_state: None,
        }
    }
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
                    InputValueConfig::Float { value } => ValueType::Float { value: *value },
                    InputValueConfig::Duration { value } => ValueType::Duration {
                        value: Duration::from_secs_f32(*value),
                    },
                    InputValueConfig::File {} => ValueType::File { value: None },
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
    type UserState = SoundGraphState;
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
                if _user_state._unserializeable_state.is_none() {
                    _user_state._unserializeable_state = get_unserializeable_state();
                }
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
                    _user_state
                        ._unserializeable_state
                        .as_mut()
                        .unwrap()
                        .input_sound_wasm_future = Some((get_input_sound(), _node_id));
                }
                let mut reset_future: bool = false;
                if let Some(future) = &mut _user_state
                    ._unserializeable_state
                    .as_mut()
                    .unwrap()
                    .input_sound_wasm_future
                {
                    if future.1 == _node_id {
                        match future.0.poll() {
                            Some(poll) => {
                                match poll {
                                    Ok(poll_item) => {
                                        let cloned = poll_item.clone();
                                        *value = Some((cloned.1, cloned.0))
                                    }
                                    Err(_) => {}
                                }
                                reset_future = true;
                            }
                            None => {}
                        }
                    }
                }
                if reset_future {
                    _user_state
                        ._unserializeable_state
                        .as_mut()
                        .unwrap()
                        .input_sound_wasm_future = None;
                }
            }
        }
        Vec::new()
    }
}

impl UserResponseTrait for ActiveNodeState {}
impl NodeDataTrait for NodeData {
    type Response = ActiveNodeState;
    type UserState = SoundGraphState;
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
        let is_recording: bool = match user_state.active_node {
            ActiveNodeState::RecordingNode(x) => x == node_id,
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
        if !is_recording {
            if ui.button("⬤ Record").clicked() {
                if user_state.active_node == ActiveNodeState::NoNode {
                    responses.push(NodeResponse::User(ActiveNodeState::RecordingNode(node_id)));
                    user_state.active_modified = true;
                }
            }
            ui.label("Recording Duration");
            ui.add(DragValue::new(&mut user_state.recording_length));
        } else {
            let button =
                egui::Button::new(egui::RichText::new("Recording...").color(egui::Color32::BLACK))
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
    GraphEditorState<NodeData, DataType, ValueType, NodeDefinitionUi, SoundGraphState>;

#[derive(Serialize, Deserialize, Default)]
pub struct SoundNodeGraphSavedState {
    pub user_state: SoundGraphState,
    pub editor_state: SoundGraphEditorState,
}

#[derive(Default)]
pub struct UnserializeableGraphState {
    pub node_definitions: Option<NodeDefinitions>,
    pub _stream: Option<(OutputStream, OutputStreamHandle)>,
    pub sink: Option<Sink>,
}

pub fn get_unserializeable_graph_state() -> UnserializeableGraphState {
    let (stream, stream_handle) =
        OutputStream::try_default().expect("could not initialize audio subsystem");
    return UnserializeableGraphState {
        node_definitions: Some(get_nodes()),
        sink: Some(Sink::try_new(&stream_handle).expect("could not create audio sink")),
        _stream: Some((stream, stream_handle)),
    };
}

#[derive(Serialize, Deserialize, Default)]
pub struct SoundNodeGraph {
    pub settings_state: WorkingFileSettings,
    pub state: SoundNodeGraphSavedState,
    pub exe_dir: String,
    #[serde(skip)]
    pub _unserializeable_state: UnserializeableGraphState,
    settings_path: String,
}

fn get_unserializeable_state() -> Option<UnserializeableState> {
    return Some(UnserializeableState {
        save_as_wasm_future: None,
        open_project_file_wasm_future: None,
        input_sound_wasm_future: None,
    });
}

impl SoundNodeGraph {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let exe_dir = get_current_exe_dir().expect("could not get app directory");
        let settings_path = std::path::Path::new(&exe_dir).join("node_settings.ron");

        let settings_state = {
            #[cfg(not(target_arch = "wasm32"))]
            match get_current_working_settings(&settings_path.to_str().unwrap_or("")) {
                Err(_) => WorkingFileSettings::default(),
                Ok(x) => x,
            }
            #[cfg(target_arch = "wasm32")]
            WorkingFileSettings::default()
        };
        Self {
            settings_path: settings_path.to_str().unwrap_or("").to_string(),
            exe_dir: exe_dir,
            state: SoundNodeGraphSavedState {
                editor_state: match &settings_state.latest_saved_file {
                    None => SoundGraphEditorState::default(),
                    Some(x) => match get_project_file(&x) {
                        Ok(y) => y.graph_state,
                        Err(_) => SoundGraphEditorState::default(),
                    },
                },
                user_state: SoundGraphState {
                    _unserializeable_state: get_unserializeable_state(),
                    ..Default::default()
                },
            },
            _unserializeable_state: get_unserializeable_graph_state(),
            settings_state: settings_state,
        }
    }

    fn save_project_file(&mut self, path: &str) {
        let _ = save_project_file(
            ProjectFile {
                graph_state: self.state.editor_state.clone(),
            },
            &path,
        );
        if path
            == self
                .settings_state
                .latest_saved_file
                .clone()
                .unwrap_or("".to_string())
        {
            self.state.user_state.is_saved = true;
        }
    }

    fn save_project_settings(&mut self, new_file: String) {
        self.settings_state.latest_saved_file = Some(new_file);
        #[cfg(not(target_arch = "wasm32"))]
        let _ = save_current_working_settings(&self.settings_path, self.settings_state.clone());
    }

    fn poll_wasm_futures(&mut self) {
        let mut option_res_2: Option<String> = None;
        if let Some(future) = &mut self
            .state
            .user_state
            ._unserializeable_state
            .as_mut()
            .unwrap()
            .save_as_wasm_future
        {
            match future.poll() {
                Some(poll) => {
                    match poll {
                        Ok(poll_item) => {
                            option_res_2 = Some(poll_item.clone());
                        }
                        Err(_) => {}
                    }
                    self.state
                        .user_state
                        ._unserializeable_state
                        .as_mut()
                        .unwrap()
                        .save_as_wasm_future = None;
                }
                None => {}
            }
        }
        match option_res_2 {
            Some(x) => {
                self.save_project_settings(x);
            }
            None => {}
        }

        let mut option_res: Option<(String, ProjectFile)> = None;
        if let Some(future) = &mut self
            .state
            .user_state
            ._unserializeable_state
            .as_mut()
            .unwrap()
            .open_project_file_wasm_future
        {
            match future.poll() {
                Some(poll) => {
                    match poll {
                        Ok(poll_item) => {
                            option_res = Some(poll_item.clone());
                        }
                        Err(_) => {}
                    }
                    self.state
                        .user_state
                        ._unserializeable_state
                        .as_mut()
                        .unwrap()
                        .open_project_file_wasm_future = None;
                }
                None => {}
            }
        }
        match option_res {
            Some(file) => {
                self.save_project_settings(file.0.clone());
                self.state = SoundNodeGraphSavedState {
                    editor_state: file.1.graph_state.clone(),
                    user_state: SoundGraphState::default(),
                };
            }
            None => {}
        }
    }

    fn save_project_settings_as(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.state
            .user_state
            ._unserializeable_state
            .as_mut()
            .unwrap()
            .save_as_wasm_future = Some(save_project_file_as(ProjectFile {
            graph_state: self.state.editor_state.clone(),
        }));
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
                match self.settings_state.latest_saved_file.clone() {
                    Some(x) => self.save_project_file(&x),
                    None => self.save_project_settings_as()?,
                }
            }
            if ui.add(egui::Button::new("Save As")).clicked() {
                self.save_project_settings_as()?;
            }
            if ui.add(egui::Button::new("Open")).clicked() {
                self.state
                    .user_state
                    ._unserializeable_state
                    .as_mut()
                    .unwrap()
                    .open_project_file_wasm_future = Some(open_project_file());
            }
            Ok(())
        });
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
        if self.state.user_state._unserializeable_state.is_none() {
            self.state.user_state._unserializeable_state = get_unserializeable_state();
        }
        if self._unserializeable_state.node_definitions.is_none() {
            self._unserializeable_state = get_unserializeable_graph_state();
        }
        self.poll_wasm_futures();
        ctx.input_mut(|i| {
            if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, egui::Key::S)) {
                match self.settings_state.latest_saved_file.clone() {
                    Some(x) => self.save_project_file(&x),
                    None => {}
                }
            }
        });
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                self.combobox(ui);
                ui.add(egui::Label::new(crate_version!()));
                ui.add(egui::Label::new("|"));
                if ui.add(egui::Link::new("tutorial")).clicked() {
                    let _url = "https://www.youtube.com/watch?v=HQrrGoOnNys";
                    #[cfg(feature = "non-wasm")]
                    let _ = open::that(_url);
                    #[cfg(target_arch = "wasm32")]
                    open_url(_url);
                }
                ui.add(egui::Label::new("|"));
                ui.add(egui::Label::new(format!(
                    "{}{}",
                    match &self.settings_state.latest_saved_file {
                        Some(x) => Path::new(x)
                            .file_name()
                            .unwrap_or(&OsStr::new(""))
                            .to_str()
                            .unwrap_or(""),
                        None => "<new project>",
                    },
                    match self.state.user_state.is_saved {
                        true => "",
                        false => "*",
                    }
                )));
            });
        });
        let mut user_state = self.state.user_state.clone();
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
                    ),
                    &mut user_state,
                    Vec::default(),
                )
            })
            .inner;
        self.state.user_state = user_state;
        if self.state.user_state._unserializeable_state.is_none() {
            self.state.user_state._unserializeable_state = get_unserializeable_state();
        }

        for node_response in graph_response.node_responses {
            if let NodeResponse::User(user_event) = node_response {
                self.state.user_state.active_node = user_event;
            }
        }

        let mut sound_result = None;

        if self.state.user_state.active_modified {
            match self.state.user_state.active_node {
                ActiveNodeState::PlayingNode(node) => {
                    if self.state.editor_state.graph.nodes.contains_key(node) {
                        let text;

                        match evaluate_node(
                            &self.state.editor_state.graph,
                            node,
                            &mut HashMap::new(),
                            &self
                                ._unserializeable_state
                                .node_definitions
                                .as_ref()
                                .unwrap(),
                        ) {
                            Ok(value) => {
                                let sound =
                                    value.try_to_source().expect("expected valid audio source");
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
                        self.state.user_state.active_node = ActiveNodeState::NoNode;
                    }
                }
                ActiveNodeState::RecordingNode(node) => {
                    if self.state.editor_state.graph.nodes.contains_key(node) {
                        let text;

                        match evaluate_node(
                            &self.state.editor_state.graph,
                            node,
                            &mut HashMap::new(),
                            &self
                                ._unserializeable_state
                                .node_definitions
                                .as_ref()
                                .unwrap(),
                        ) {
                            Ok(value) => {
                                let sound =
                                    value.try_to_source().expect("expected valid audio source");
                                sound_result = Some(sound.clone());
                                text = "Recording Anonymous audio source.";
                            }
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
                        self.state.user_state.active_node = ActiveNodeState::NoNode;
                    }
                }
                ActiveNodeState::NoNode => {}
            }
        }

        match sound_result {
            Some(x) => match self.state.user_state.active_node {
                ActiveNodeState::PlayingNode(_) => {
                    if self.state.user_state.active_modified {
                        sound_map::set_repeats(x, 1);
                        let sound: sound_map::RefSource = match sound_map::clone_sound(x) {
                            Err(_) => {
                                let x = sound_map::push_sound::<Zero<f32>>(Box::new(
                                    Zero::<f32>::new(1, DEFAULT_SAMPLE_RATE),
                                ));
                                sound_map::clone_sound(x).unwrap()
                            }
                            Ok(x) => x,
                        };
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let sink = self._unserializeable_state.sink.as_ref().unwrap();
                            sink.append(sound);
                            sink.play();
                            sink.set_volume(1.0);
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            let channels = 2;
                            let mut length = 1 + self.state.user_state.recording_length as u32;
                            let context = AudioContext::new().expect("wasm audio failed");
                            let sample_rate = context.sample_rate().round() as u32;
                            let mut translated_sound: UniformSourceIterator<RefSource, f32> =
                                UniformSourceIterator::new(sound, channels, sample_rate);
                            context.destination().set_channel_count(channels.into());

                            let mut buffer = context
                                .create_buffer(
                                    channels.into(),
                                    sample_rate * length,
                                    sample_rate as f32,
                                )
                                .expect("wasm audio failed");
                            let mut buffer_values_0 = vec![];
                            let mut buffer_values_1 = vec![];
                            let mut flip: bool = false;
                            for i in 0..sample_rate * length * channels as u32 {
                                match translated_sound.next() {
                                    Some(f) => {
                                        flip = !flip;
                                        if flip {
                                            buffer_values_0.push(f);
                                        } else {
                                            buffer_values_1.push(f);
                                        }
                                    }
                                    None => {
                                        break;
                                    }
                                }
                            }
                            buffer
                                .copy_to_channel(&buffer_values_0, 0)
                                .expect("wasm audio failed");
                            buffer
                                .copy_to_channel(&buffer_values_1, 1)
                                .expect("wasm audio failed");

                            let mut src =
                                context.create_buffer_source().expect("wasm audio failed");
                            src.set_buffer(Some(&buffer));
                            src.connect_with_audio_node(&context.destination())
                                .expect("wasm audio failed");
                            src.start().expect("wasm audio failed");
                            let _ = context.resume().expect("wasm audio failed");
                        }

                        self.state.user_state.active_modified = false;
                    }
                }
                ActiveNodeState::RecordingNode(_) => {
                    if self.state.user_state.active_modified
                        && !self.state.user_state.is_done_showing_recording_dialogue
                    {
                        sound_map::set_repeats(x, 1);
                        let source = match sound_map::clone_sound(x) {
                            Err(_) => {
                                let x = sound_map::push_sound::<Zero<f32>>(Box::new(Zero::new(
                                    1,
                                    DEFAULT_SAMPLE_RATE,
                                )));
                                sound_map::clone_sound(x).unwrap()
                            }
                            Ok(x) => x,
                        };
                        let spec = hound::WavSpec {
                            channels: source.channels(),
                            sample_rate: DEFAULT_SAMPLE_RATE,
                            bits_per_sample: 32,
                            sample_format: hound::SampleFormat::Float,
                        };
                        let recording_len =
                            self.state.user_state.recording_length * DEFAULT_SAMPLE_RATE as usize;
                        let mut vec = Vec::with_capacity(recording_len);
                        let cursor = Cursor::new(&mut vec);
                        let stream = BufWriter::new(cursor);
                        let mut writer = hound::WavWriter::new(stream, spec).unwrap();
                        for (idx, sample) in source.enumerate() {
                            if idx / DEFAULT_SAMPLE_RATE as usize
                                > self.state.user_state.recording_length
                            {
                                break;
                            }
                            let _ = writer.write_sample(sample).unwrap();
                        }
                        let _ = writer.finalize().unwrap();
                        let _ = write_output_sound(vec);
                        self.state.user_state.is_done_showing_recording_dialogue = true;
                    }
                }
                ActiveNodeState::NoNode => {
                    if self.state.user_state.active_modified == true {
                        self._unserializeable_state.sink.as_ref().unwrap().clear();
                        sound_map::clear();
                        self.state.user_state.active_modified = false;
                        self.state.user_state.is_done_showing_recording_dialogue = false;
                    }
                }
            },
            None => {
                if self.state.user_state.active_modified == true {
                    self._unserializeable_state.sink.as_ref().unwrap().clear();
                    sound_map::clear();
                    self.state.user_state.active_modified = false;
                    self.state.user_state.is_done_showing_recording_dialogue = false;
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
