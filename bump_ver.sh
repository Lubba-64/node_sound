cd node_sound_vst
cargo bump "$1" 
cd ../node_sound_core
cargo bump "$1"
cd ../node_sound_app
cargo bump "$1"
