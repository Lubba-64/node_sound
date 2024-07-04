# node_sound

A node based sound construction program written in rust.

I reserve the right to quit this project whenever, that said I am still finding fun new things to add every now and then...

Install/Compile instructions:

A web demo is available at https://lubba-64.github.io/ if you want to skip the install process.

If you want the windows VST, a precompiled release will be available in `releases`, for every other platform / distributable, follow the given install instructions.

 - install [rustup](https://www.rust-lang.org/tools/install)
 - clone the repository with `git clone <repo>` [git](https://git-scm.com/downloads)
 - run `cargo run --bin xtask bundle node_sound_vst --release` in the root of the repo to generate a VST and CLAP plugin for your OS of choice.
 - run `cargo run --bin node_sound_app --release --features non-wasm` in the root of the repo to generate a standalone desktop app for your OS of choice.
 - run `cd  node_sound_app`, `cargo install trunk` and `trunk serve` to run a dev build of the web version.

