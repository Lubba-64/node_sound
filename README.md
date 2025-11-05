# node_sound

A node based sound construction program written in rust.

I have completed 99% of what I have set out to do with this project, and then some. Future updates will still likely occur, because this is fun to mess with...

Install/Compile instructions:

A web demo is available at https://lubba-64.github.io/ if you want to skip the install process. It's out of date because turning this into just a VST was much easier and better looking for the codebase. old versions are still in the git tree.

If you want the windows VST, a precompiled release will be available in `releases`, for every other platform / distributable, follow the given install instructions.

 - install [rustup](https://www.rust-lang.org/tools/install)
 - clone the repository with `git clone <repo>` [git](https://git-scm.com/downloads)
 - run `cargo run --bin xtask bundle node_sound_vst --release` in the root of the repo to generate a VST and CLAP plugin for your OS of choice for the standard synth plugin.
 - run `cargo run --bin xtask bundle node_sound_vst_effect --release` in the root of the repo to generate a VST and CLAP plugin for your OS of choice for the effect plugin.