#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod nodes;
mod sound_graph;
mod sound_map;
mod sounds;
use eframe;
use eframe::egui;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> () {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Sound node graph",
        native_options,
        Box::new(|cc| Box::new(sound_graph::graph::SoundNodeGraph::new(cc))),
    )
    .expect("eframe failed to run");
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(sound_graph::graph::SoundNodeGraph::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
