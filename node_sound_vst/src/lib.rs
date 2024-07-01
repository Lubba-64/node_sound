use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, EguiState};
use node_sound_core::sound_graph;
use std::sync::Arc;
pub struct NodeSound {
    params: Arc<NodeSoundParams>,
}

#[derive(Params)]
pub struct NodeSoundParams {
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,
}

impl Default for NodeSound {
    fn default() -> Self {
        Self {
            params: Arc::new(NodeSoundParams::default()),
        }
    }
}

impl Default for NodeSoundParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(1280, 720),
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
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        create_egui_editor(
            self.params.editor_state.clone(),
            sound_graph::graph::SoundNodeGraph::new_raw(),
            |_, _| {},
            move |egui_ctx, _setter, state| state.update_root(egui_ctx),
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
        for channel_samples in buffer.iter_samples() {}

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
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Synth, Vst3SubCategory::Generator];
}

nih_export_clap!(NodeSound);
nih_export_vst3!(NodeSound);
