use egui_extras_xt::knobs::AudioKnob;
use nih_plug::{params::persist::PersistentField, prelude::*};
use nih_plug_egui::{EguiState, create_egui_editor};
use node_sound_core::sound_graph::graph::FileManager;
use node_sound_core::sound_map::DawSource;
use node_sound_core::{
    sound_graph::{
        self,
        graph::{ActiveNodeState, SoundNodeGraph, evaluate_node},
    },
    sound_map::GenericSource,
};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct NodeSound {
    params: Arc<NodeSoundParams>,
    sample_rate: Arc<Mutex<f32>>,
    bpm: Arc<Mutex<f32>>,
    sound_result: Arc<Mutex<Option<GenericSource>>>,
    total_idx: usize,
}

pub struct PluginPresetState {
    graph: Arc<Mutex<SoundNodeGraph>>,
}

#[derive(Params)]
pub struct NodeSoundParams {
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,
    #[persist = "editor-preset"]
    plugin_state: PluginPresetState,
    #[id = "a1"]
    pub a1: FloatParam,
    #[id = "a2"]
    pub a2: FloatParam,
    #[id = "a3"]
    pub a3: FloatParam,
    #[id = "a4"]
    pub a4: FloatParam,
    #[id = "a5"]
    pub a5: FloatParam,
    #[id = "a6"]
    pub a6: FloatParam,
    #[id = "a7"]
    pub a7: FloatParam,
    #[id = "a8"]
    pub a8: FloatParam,
    #[id = "a9"]
    pub a9: FloatParam,
    #[id = "a10"]
    pub a10: FloatParam,
    #[id = "a11"]
    pub a11: FloatParam,
    #[id = "a12"]
    pub a12: FloatParam,
    #[id = "a13"]
    pub a13: FloatParam,
    #[id = "a14"]
    pub a14: FloatParam,
    #[id = "a15"]
    pub a15: FloatParam,
    #[id = "a16"]
    pub a16: FloatParam,
    #[id = "a17"]
    pub a17: FloatParam,
    #[id = "a18"]
    pub a18: FloatParam,
}

impl<'a> PersistentField<'a, String> for PluginPresetState {
    fn set(&self, new_value: String) {
        *self.graph.lock().expect("expected to lock") =
            ron::de::from_str(&new_value).expect("expect deserialize");
    }

    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&String) -> R,
    {
        f(&ron::ser::to_string(&self.graph).expect("Graph state is ok"))
    }
}

impl Default for NodeSound {
    fn default() -> Self {
        let params = NodeSoundParams::default();
        Self {
            params: Arc::new(params),
            sample_rate: Arc::new(Mutex::new(48000.0)),
            bpm: Arc::new(Mutex::new(120.0)),
            sound_result: Arc::new(Mutex::new(None)),
            total_idx: 0,
        }
    }
}

macro_rules! mkparam {
    ($field: ident, $name: literal) => {
        let $field = FloatParam::new(
            $name,
            0.0,
            FloatRange::Linear {
                min: -1.0,
                max: 1.0,
            },
        )
        .with_smoother(SmoothingStyle::None)
        .with_step_size(0.01);
    };
}

impl Default for NodeSoundParams {
    fn default() -> Self {
        mkparam! {a1, "A1"}
        mkparam! {a2, "A2"}
        mkparam! {a3, "A3"}
        mkparam! {a4, "A4"}
        mkparam! {a5, "A5"}
        mkparam! {a6, "A6"}
        mkparam! {a7, "A7"}
        mkparam! {a8, "A8"}
        mkparam! {a9, "A9"}
        mkparam! {a10, "A10"}
        mkparam! {a11, "A11"}
        mkparam! {a12, "A12"}
        mkparam! {a13, "A13"}
        mkparam! {a14, "A14"}
        mkparam! {a15, "A15"}
        mkparam! {a16, "A16"}
        mkparam! {a17, "A17"}
        mkparam! {a18, "A18"}

        Self {
            editor_state: EguiState::from_size(1280, 720),
            plugin_state: PluginPresetState {
                graph: Arc::new(Mutex::new(
                    sound_graph::graph::SoundNodeGraph::new_vst_effect(),
                )),
            },
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8,
            a9,
            a10,
            a11,
            a12,
            a13,
            a14,
            a15,
            a16,
            a17,
            a18,
        }
    }
}

macro_rules! mkparamgetter {
    ($field: ident, $idx: literal, $self: ident, $buff: ident) => {
        let $field = $self.params.$field.value();
        match $buff[$idx].lock() {
            Ok(mut x) => {
                *x = $field;
            }
            Err(_x) => {}
        }
    };
}

pub enum BackgroundTasks {
    MidiFileOpen(Arc<Mutex<FileManager>>),
    WavFileOpen(Arc<Mutex<FileManager>>),
}

impl Plugin for NodeSound {
    const NAME: &'static str = "Node Sound Effect";
    const VENDOR: &'static str = "Lubba64";
    const URL: &'static str = "https://lubba-64.github.io/";
    const EMAIL: &'static str = "Lubba64@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    type SysExMessage = ();
    type BackgroundTask = BackgroundTasks;

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn task_executor(&mut self) -> TaskExecutor<Self> {
        Box::new(|cx| match cx {
            BackgroundTasks::MidiFileOpen(files) => {
                match files.lock() {
                    Err(_x) => {}
                    Ok(mut x) => {
                        match x.midi_active {
                            None => {}
                            Some(node_id) => {
                                x.midi_file_path = Some((
                                    rfd::FileDialog::new()
                                        .add_filter("audio", &["mid", "midi"])
                                        .pick_file()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default()
                                        .to_string(),
                                    node_id,
                                ));
                                x.midi_active = None;
                            }
                        };
                    }
                };
            }
            BackgroundTasks::WavFileOpen(files) => {
                match files.lock() {
                    Err(_x) => {}
                    Ok(mut x) => {
                        match x.wav_active {
                            None => {}
                            Some(node_id) => {
                                x.wav_file_path = Some((
                                    rfd::FileDialog::new()
                                        .add_filter("audio", &["wav", "mp3", "flac", "ogg"])
                                        .pick_file()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default()
                                        .to_string(),
                                    node_id,
                                ));
                                x.wav_active = None;
                            }
                        };
                    }
                };
            }
        })
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        create_egui_editor(
            self.params.editor_state.clone(),
            (
                self.params.plugin_state.graph.clone(),
                self.sound_result.clone(),
                self.sample_rate.clone(),
                self.bpm.clone(),
                self.params.clone(),
                false,
                None,
            ),
            |_, _| {},
            move |egui_ctx, setter, state| {
                let sample_rate = &mut match state.2.lock() {
                    Ok(x) => x,
                    Err(_x) => {
                        return;
                    }
                };
                let graph = &mut match state.0.lock() {
                    Ok(x) => x,
                    Err(_x) => {
                        return;
                    }
                };
                let error: &mut Option<String> = &mut state.6;
                egui::TopBottomPanel::bottom("automations").show(egui_ctx, |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.label("Automations: ");
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let params = &state.4;
                                let knobs = [
                                    ("A1", &params.a1),
                                    ("A2", &params.a2),
                                    ("A3", &params.a3),
                                    ("A4", &params.a4),
                                    ("A5", &params.a5),
                                    ("A6", &params.a6),
                                    ("A7", &params.a7),
                                    ("A8", &params.a8),
                                    ("A9", &params.a9),
                                    ("A10", &params.a10),
                                    ("A11", &params.a11),
                                    ("A12", &params.a12),
                                    ("A13", &params.a13),
                                    ("A14", &params.a14),
                                    ("A15", &params.a15),
                                    ("A16", &params.a16),
                                    ("A17", &params.a17),
                                    ("A18", &params.a18),
                                ];

                                for (label, param) in knobs.iter() {
                                    ui.vertical(|ui| {
                                        ui.label(*label);
                                        let param_value = param.value();
                                        let mut current_value = param_value;
                                        let response = ui.add(
                                            AudioKnob::new(&mut current_value)
                                                .range(-1.0..=1.0)
                                                .drag_length(50.0)
                                                .diameter(15.0),
                                        );
                                        if response.changed() && current_value != param_value {
                                            setter.set_parameter(*param, current_value);
                                        }
                                    });
                                    ui.add_space(2.0);
                                }
                                ui.separator();
                                match error {
                                    Some(err) => {
                                        ui.label(format!("Error: {}", err));
                                    }
                                    None => {}
                                }
                            });
                        });
                    });
                });
                graph.update_root(egui_ctx);
                if !state.5 || graph.state.user_state.active_node.is_playing() {
                    graph.state.user_state.active_node = ActiveNodeState::NoNode;
                    match graph.state.user_state.vst_output_node_id {
                        Some(x) => {
                            graph
                                .state
                                ._unserializeable_state
                                .queue
                                .set_sample_rate(**sample_rate);
                            graph.state.user_state.wavetables.clear();
                            graph.state._unserializeable_state.queue.set_note_speed(1.0);
                            graph
                                .state
                                ._unserializeable_state
                                .queue
                                .set_bpm(state.3.clone());
                            match evaluate_node(
                                &graph.state.editor_state.graph.clone(),
                                x,
                                &mut HashMap::new(),
                                &graph.state._unserializeable_state.node_definitions.clone(),
                                &mut graph.state,
                            ) {
                                Ok(val) => {
                                    let source_id = match val.try_to_source() {
                                        Err(_x) => return,
                                        Ok(x) => x,
                                    }
                                    .clone();
                                    let sound = match graph
                                        .state
                                        ._unserializeable_state
                                        .queue
                                        .clone_sound(source_id.clone())
                                    {
                                        Err(_err) => {
                                            return;
                                        }
                                        Ok(x) => x,
                                    };
                                    let sound_result = &mut match state.1.lock() {
                                        Ok(x) => x,
                                        Err(_x) => {
                                            return;
                                        }
                                    };
                                    **sound_result = Some(sound);
                                    state.5 = true;
                                }
                                Err(err) => {
                                    *error = Some(format!("{:?}", err));
                                    graph.state._unserializeable_state.queue.clear();
                                }
                            };
                        }
                        None => {
                            graph.state._unserializeable_state.queue.clear();
                        }
                    }
                }
            },
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        match self.sample_rate.try_lock() {
            Ok(mut x) => *x = context.transport().sample_rate,
            Err(_x) => {}
        };
        let state = &match self.params.plugin_state.graph.lock() {
            Ok(x) => x,
            Err(_x) => {
                return ProcessStatus::KeepAlive;
            }
        }
        .state;
        match self.bpm.try_lock() {
            Ok(mut x) => {
                *x = context.transport().tempo.unwrap_or(120.0) as f32;
            }
            Err(_x) => {}
        };
        match state.user_state.files.try_lock() {
            Ok(x) => {
                if x.midi_active.is_some() {
                    context.execute_background(BackgroundTasks::MidiFileOpen(
                        state.user_state.files.clone(),
                    ));
                }
                if x.wav_active.is_some() {
                    context.execute_background(BackgroundTasks::WavFileOpen(
                        state.user_state.files.clone(),
                    ));
                }
            }
            Err(_x) => {}
        }
        let automations = state._unserializeable_state.automations.0.clone();
        let input = state._unserializeable_state.input.0.clone();
        let size = buffer.samples();
        let output = buffer.as_slice();
        let mut sound_result = match self.sound_result.try_lock() {
            Ok(x) => x,
            Err(_x) => {
                return ProcessStatus::KeepAlive;
            }
        }
        .clone();
        for sample_idx in 0..size {
            match input.try_lock() {
                Ok(mut x) => {
                    x.0 = output[0][sample_idx];
                    x.1 = output[1][sample_idx];
                }
                _ => {}
            }
            output[0][sample_idx] = 0.0;
            output[1][sample_idx] = 0.0;
            mkparamgetter!(a1, 0, self, automations);
            mkparamgetter!(a2, 1, self, automations);
            mkparamgetter!(a3, 2, self, automations);
            mkparamgetter!(a4, 3, self, automations);
            mkparamgetter!(a5, 4, self, automations);
            mkparamgetter!(a6, 5, self, automations);
            mkparamgetter!(a7, 6, self, automations);
            mkparamgetter!(a8, 7, self, automations);
            mkparamgetter!(a9, 8, self, automations);
            mkparamgetter!(a10, 9, self, automations);
            mkparamgetter!(a11, 10, self, automations);
            mkparamgetter!(a12, 11, self, automations);
            mkparamgetter!(a13, 12, self, automations);
            mkparamgetter!(a14, 13, self, automations);
            mkparamgetter!(a15, 14, self, automations);
            mkparamgetter!(a16, 15, self, automations);
            mkparamgetter!(a17, 16, self, automations);
            mkparamgetter!(a18, 17, self, automations);
            match &mut sound_result {
                Some(source) => {
                    let time_index = (sample_idx + self.total_idx) as f32;
                    let left_sample = source.next(time_index, 0).unwrap_or_default();
                    let right_sample = source.next(time_index, 1).unwrap_or_default();
                    output[0][sample_idx] = left_sample.clamp(-1.0, 1.0);
                    output[1][sample_idx] = right_sample.clamp(-1.0, 1.0);
                }
                None => {
                    output[0][sample_idx] = 0.0;
                    output[1][sample_idx] = 0.0;
                }
            }
        }
        self.total_idx = self.total_idx.wrapping_add(size);

        ProcessStatus::Normal
    }
}

impl ClapPlugin for NodeSound {
    const CLAP_ID: &'static str = "com.lubba64-plugins-egui.node-sound-egui-effect";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A node graph waveform synth");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Instrument,
    ];
}

impl Vst3Plugin for NodeSound {
    const VST3_CLASS_ID: [u8; 16] = *b"NodeSoun2Lubba64";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Stereo];
}

nih_export_clap!(NodeSound);
nih_export_vst3!(NodeSound);
