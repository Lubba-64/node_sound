mod nodes;
mod sound_graph;
mod sound_queue;
mod sounds;
use eframe::egui::Visuals;

fn main() {
    eframe::run_native(
        "Egui node graph example",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            Box::new(sound_graph::graph::SoundNodeGraph::new())
        }),
    )
    .expect("eframe failed to run");
}
