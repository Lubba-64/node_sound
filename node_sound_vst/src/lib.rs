use nih_plug::{params::persist::PersistentField, prelude::*};
use nih_plug_egui::{create_egui_editor, EguiState};
use node_sound_core::{
    sound_graph::{
        self,
        graph::{evaluate_node, SoundNodeGraph},
    },
    sound_map::{self, RefSource},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct NodeSound {
    params: Arc<NodeSoundParams>,
    sound: Arc<Mutex<Option<RefSource>>>,
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
        Self {
            params: Arc::new(NodeSoundParams::default()),
            sound: Arc::new(Mutex::new(None)),
        }
    }
}

impl Default for NodeSoundParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(1280, 720),
            plugin_state: PluginPresetState {
                graph: Arc::new(Mutex::new(sound_graph::graph::SoundNodeGraph::new_raw())),
            },
        }
    }
}

impl Plugin for NodeSound {
    const NAME: &'static str = "Node Sound";
    const VENDOR: &'static str = "Lubba64";
    const URL: &'static str = "https://lubba-64.github.io/";
    const EMAIL: &'static str = "Lubba64@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        create_egui_editor(
            self.params.editor_state.clone(),
            (self.params.plugin_state.graph.clone(), self.sound.clone()),
            |_, _| {},
            move |egui_ctx, _setter, state| {
                let sound_result = &mut state.1.lock().expect("");
                let state = &mut state.0.lock().expect("");

                state.update_root(egui_ctx);
                if sound_result.is_none() || state.state.user_state.active_node.is_playing() {
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
                                    let sound: sound_map::RefSource = match sound_map::clone_sound(
                                        val.try_to_source()
                                            .expect("expected valid audio source")
                                            .clone(),
                                    ) {
                                        Err(_err) => {
                                            return;
                                        }
                                        Ok(x) => x,
                                    };
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
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for mut channel_samples in buffer.iter_samples() {
            for sample in channel_samples.iter_mut() {
                *sample = match &mut self.sound.lock().expect("expected lock on source").clone() {
                    Some(x) => x.next().unwrap_or(0.0).clamp(-1.0, 1.0),
                    None => 0.0,
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for NodeSound {
    const CLAP_ID: &'static str = "com.lubba64-plugins-egui.node-sound-egui";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A node graph waveform synth");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
        ClapFeature::Instrument,
    ];
}

impl Vst3Plugin for NodeSound {
    const VST3_CLASS_ID: [u8; 16] = *b"NodeSoundLubba64";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Stereo,
    ];
}

nih_export_clap!(NodeSound);
nih_export_vst3!(NodeSound);
