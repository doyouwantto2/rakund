use crate::engine::decoder;
use crate::events::orchestrator;
use crate::models::SplendidConfig;
use crate::setup;
use std::collections::HashMap;
use std::fs;

pub fn run() {
    let audio_handle = setup::sound::start_stream();

    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

    let config_path = cwd.join("data/map/splendid.json");

    let samples_base = cwd.join("data/splendid/Samples");

    let config_data = fs::read_to_string(&config_path)
        .expect(&format!("❌ Could not find JSON at {:?}", config_path));

    let mut piano_config: SplendidConfig =
        serde_json::from_str(&config_data).expect("Invalid JSON");

    println!("📦 Loading samples from: {:?}", samples_base);
    let mut cache = HashMap::new();

    for key_data in piano_config.keys.values() {
        for sample in &key_data.samples {
            if !cache.contains_key(&sample.file) {
                let sample_path = samples_base.join(&sample.file);

                match decoder::decode_flac(sample_path.to_str().unwrap_or("")) {
                    Ok(data) => {
                        cache.insert(sample.file.clone(), data);
                        println!("  ✅ Loaded: {}", sample.file);
                    }
                    Err(e) => {
                        eprintln!("  ❌ Failed at {:?}: {}", sample_path, e);
                    }
                }
            }
        }
    }

    piano_config.samples_cache = cache;
    println!(
        "🎹 RAM Loading Complete. {} samples ready.",
        piano_config.samples_cache.len()
    );

    tauri::Builder::default()
        .manage(audio_handle)
        .manage(piano_config)
        .invoke_handler(tauri::generate_handler![
            orchestrator::play_midi_note,
            orchestrator::stop_midi_note,
            orchestrator::set_sustain,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
