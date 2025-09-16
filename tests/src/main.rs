use node_sound_core::sound_map::DawSource;
use node_sound_core::sounds::sine::SineWave;
use node_sound_core::sounds::vertical_wave_shaper::VerticalWaveShaper;
use rand::Rng;

fn main() {
    // Create a random noise table with 25 elements
    let mut rng = rand::thread_rng();
    let table: Vec<f32> = (0..25).map(|_| rng.gen_range(-1.0..1.0)).collect();

    println!("Wave shaping table ({} elements):", table.len());
    for (i, value) in table.iter().enumerate() {
        println!("  [{}]: {:.6}", i, value);
    }
    println!();

    // Create the wave shaper
    let mut shaper = VerticalWaveShaper::new(SineWave::new(440.0, false), table);

    // Test with a sine wave at various phases
    println!("Testing VerticalWaveShaper with sine wave input:");
    println!("{:<8} | {:<12} | {:<12}", "Index", "Input", "Output");
    println!("{}", "-".repeat(35));

    let sample_rate = 44100.0;
    let num_samples = 20;

    for i in 0..num_samples {
        let index = i as f32 / sample_rate;
        let phase = (i as f32 / num_samples as f32) * 2.0 * std::f32::consts::PI;
        let input_value = phase.sin(); // Generate sine wave manually

        // Get the output from the wave shaper
        if let Some(output) = shaper.next(index, 0) {
            println!("{:<8.4} | {:<12.6} | {:<12.6}", index, input_value, output);
        }
    }

    // Test edge cases
    println!("\nTesting edge cases:");
    println!("{:<8} | {:<12} | {:<12}", "Input", "Normalized", "Output");
    println!("{}", "-".repeat(35));
}
