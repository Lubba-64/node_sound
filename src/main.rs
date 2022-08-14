use rodio::{OutputStream, Source};
use std::time::Duration;
mod sources;
use sources::*;
mod sound_graph;

fn main() {
    let mut source = DefaultSource::new(44100, sin_wave_table(64), Some(Duration::new(1, 0)));
    let mut source2 = DefaultSource::new(44100, square_wave_table(64), Some(Duration::new(1, 0)));
    source2.set_frequency(442.0);
    source.set_frequency(100.0);

    let source_ops = source2
        .amplify(0.05)
        .fade_in(Duration::new(1, 0))
        .mix(source.amplify(0.01))
        .reverb(Duration::new(0, 10), 0.2)
        .amplify(0.2)
        .speed(0.3)
        .take_duration(Duration::new(1, 0));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(source_ops.convert_samples());

    use eframe::egui::Visuals;

    eframe::run_native(
        "Egui node graph example",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            Box::new(sound_graph::graph::NodeGraphExample::default())
        }),
    );
}
