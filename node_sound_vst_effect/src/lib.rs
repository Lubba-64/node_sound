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

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct NodeSound {
    params: Arc<NodeSoundParams>,
    sample_rate: Arc<Mutex<f32>>,
    sound_result: Arc<Mutex<Option<GenericSource<f32>>>>,
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
    /// A voice's gain. This can be polyphonically modulated.
    #[id = "gain"]
    gain: FloatParam,
    /// The amplitude envelope attack time. This is the same for every voice.
    #[id = "amp_atk"]
    amp_attack_ms: FloatParam,
    /// The amplitude envelope release time. This is the same for every voice.
    #[id = "amp_rel"]
    amp_release_ms: FloatParam,
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
        *self.graph.lock().expect("expected to lock") = ron::de::from_str(&new_value).expect("");
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
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-12.0),
                // Because we're representing gain as decibels the range is already logarithmic
                FloatRange::Linear {
                    min: util::db_to_gain(-36.0),
                    max: util::db_to_gain(0.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(5.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            amp_attack_ms: FloatParam::new(
                "Attack",
                200.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            amp_release_ms: FloatParam::new(
                "Release",
                100.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
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
    const NAME: &'static str = "Node Sound";
    const VENDOR: &'static str = "Lubba64";
    const URL: &'static str = "https://lubba-64.github.io/";
    const EMAIL: &'static str = "Lubba64@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

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
            ),
            |_, _| {},
            move |egui_ctx, _setter, state| {
                let sound_result = &mut state.1.lock().expect("");
                let sound_result_id = &mut state.2.lock().expect("");
                let state = &mut state.0.lock().expect("");

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

                                    **sound_result = Some(sound);
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
        let num_samples = buffer.samples();
        *self.sample_rate.lock().expect("expect lock") = context.transport().sample_rate;
        let sample_rate = context.transport().sample_rate;
        let input_output = buffer.as_slice();

        let mut samples_interleaved = Vec::with_capacity(num_samples * 2);
        for idx in 0..num_samples {
            samples_interleaved[idx * 2] = input_output[0][idx];
            samples_interleaved[idx * 2 + 1] = input_output[1][idx];
        }

        unsafe { DAW_INPUT = Some((sample_rate as u32, samples_interleaved)) }

        let mut sound_result = self.sound_result.lock().expect("expected lock");

        let mut output = [
            Vec::with_capacity(num_samples),
            Vec::with_capacity(num_samples),
        ];
        match &mut *sound_result {
            Some(x) => {
                for idx in 0..num_samples {
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
                    output[0][idx] = x.next().unwrap_or(0.0).clamp(-1.0, 1.0);
                    output[1][idx] = x.next().unwrap_or(0.0).clamp(-1.0, 1.0);
                }
            }
            None => {}
        }

        for (channel_idx, mut channel_samples) in buffer.iter_samples().enumerate() {
            for (idx, sample) in channel_samples.iter_mut().enumerate() {
                *sample = output[channel_idx][idx]
            }
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
