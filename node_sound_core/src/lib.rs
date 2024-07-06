mod macros;
pub mod nodes;
pub mod sound_graph;
pub mod sound_map;
pub mod sounds;

#[cfg(all(feature = "vst", feature = "non-wasm"))]
compile_error!("Feature 1 and 2 are mutually exclusive and cannot be enabled together");
