use eframe::egui;
use node_sound_core::sound_graph;

fn main() -> () {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sound node graph",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(sound_graph::graph::SoundNodeGraph::new_app(Some(
                cc,
            ))))
        }),
    )
    .expect("eframe failed to run");
}
