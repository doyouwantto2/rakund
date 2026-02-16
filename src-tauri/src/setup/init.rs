use crate::events::{note, orchestrator};
use crate::models::SplendidConfig;
use crate::setup;
use std::fs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let audio_handle = setup::sound::start_stream();

    let config_path = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("data/map/splendid.json");

    let config_data = fs::read_to_string(&config_path).unwrap_or_else(|e| {
        panic!(
            "Failed to load piano configuration from {:?}: {}",
            config_path, e
        )
    });

    let piano_config: SplendidConfig =
        serde_json::from_str(&config_data).expect("Invalid piano configuration JSON format");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(audio_handle)
        .manage(piano_config)
        .invoke_handler(tauri::generate_handler![
            note::play_note,
            orchestrator::play_midi_note,
            orchestrator::stop_midi_note,
            orchestrator::set_sustain,
        ])
        .run(tauri::generate_context!())
        .expect("Failed to start Raku Grand Piano application");
}
