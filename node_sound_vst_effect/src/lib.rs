use nih_plug::{params::persist::PersistentField, prelude::*};
use nih_plug_egui::{create_egui_editor, EguiState};
use node_sound_core::{
    sound_graph::{
        self,
        graph::{evaluate_node, ActiveNodeState, SoundNodeGraph},
    },
    sound_map::{self, GenericSource},
    sounds::{DAW_BUFF, DAW_INPUT},
};
use rodio::source::UniformSourceIterator;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct NodeSound {
    params: Arc<NodeSoundParams>,
    sample_rate: Arc<Mutex<f32>>,
    sound_result: Arc<Mutex<Option<UniformSourceIterator<GenericSource<f32>, f32>>>>,
    idx: u8,
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
    root_sound_id: Arc<Mutex<Option<usize>>>,
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
            sound_result: Arc::new(Mutex::new(None)),
            idx: 0,
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
            root_sound_id: Arc::new(Mutex::new(None)),
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
    ($field: ident, $idx: literal, $self: ident) => {
        let $field = $self.params.$field.value();
        unsafe {
            DAW_BUFF[$idx] = Some($field);
        }
    };
}

impl Plugin for NodeSound {
    const NAME: &'static str = "Node Sound Effect";
    const VENDOR: &'static str = "Lubba64";
    const URL: &'static str = "https://lubba-64.github.io/";
    const EMAIL: &'static str = "Lubba64@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(1),
        main_output_channels: NonZeroU32::new(1),
        ..AudioIOLayout::const_default()
    }];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        create_egui_editor(
            self.params.editor_state.clone(),
            (
                self.params.plugin_state.graph.clone(),
                self.sound_result.clone(),
                self.params.root_sound_id.clone(),
                self.sample_rate.clone(),
            ),
            |_, _| {},
            move |egui_ctx, _setter, state| {
                let sound_result = &mut state.1.lock().expect("expected lock");
                let sound_result_id = &mut state.2.lock().expect("expected lock");
                let sample_rate = &mut state.3.lock().expect("expected lock");
                let state = &mut state.0.lock().expect("expected lock");

                state.update_root(egui_ctx);
                if sound_result.is_none() || state.state.user_state.active_node.is_playing() {
                    state.state.user_state.active_node = ActiveNodeState::NoNode;
                    match state.state.user_state.vst_output_node_id {
                        Some(x) => {
                            match evaluate_node(
                                &state.state.editor_state.graph,
                                x,
                                &mut HashMap::new(),
                                &state
                                    ._unserializeable_state
                                    .node_definitions
                                    .as_ref()
                                    .unwrap(),
                            ) {
                                Ok(val) => {
                                    let source_id = val
                                        .try_to_source()
                                        .expect("expected valid audio source")
                                        .clone();
                                    let sound = match sound_map::clone_sound(source_id.clone()) {
                                        Err(_err) => {
                                            return;
                                        }
                                        Ok(x) => x,
                                    };

                                    **sound_result_id = Some(source_id);
                                    **sound_result = Some(UniformSourceIterator::new(
                                        sound,
                                        1,
                                        **sample_rate as u32,
                                    ));
                                }
                                Err(_err) => {
                                    sound_map::clear();
                                    **sound_result = None
                                }
                            };
                        }
                        None => {
                            sound_map::clear();
                            **sound_result = None
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
        *self.sample_rate.lock().expect("expect lock") = context.transport().sample_rate;
        let sample_rate = context.transport().sample_rate;
        let mut sound_result = self.sound_result.lock().expect("expected lock");
        self.idx = 0;
        let sound_result = match &mut *sound_result {
            Some(x) => x,
            None => return ProcessStatus::Normal,
        };

        for mut channel_samples in buffer.iter_samples() {
            for sample in channel_samples.iter_mut() {
                unsafe { DAW_INPUT = Some((sample_rate as u32, *sample, *sample)) }
                mkparamgetter!(a1, 0, self);
                mkparamgetter!(a2, 1, self);
                mkparamgetter!(a3, 2, self);
                mkparamgetter!(a4, 3, self);
                mkparamgetter!(a5, 4, self);
                mkparamgetter!(a6, 5, self);
                mkparamgetter!(a7, 6, self);
                mkparamgetter!(a8, 7, self);
                mkparamgetter!(a9, 8, self);
                mkparamgetter!(a10, 9, self);
                mkparamgetter!(a11, 10, self);
                mkparamgetter!(a12, 11, self);
                mkparamgetter!(a13, 12, self);
                mkparamgetter!(a14, 13, self);
                mkparamgetter!(a15, 14, self);
                mkparamgetter!(a16, 15, self);
                mkparamgetter!(a17, 16, self);
                mkparamgetter!(a18, 17, self);
                *sample = sound_result.next().unwrap_or(0.0).clamp(-1.0, 1.0);
            }
            self.idx += 1;
        }

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
