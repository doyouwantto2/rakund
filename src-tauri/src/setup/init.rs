use crate::events::audio::play_note;
use crate::setup;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup::sound::start_stream();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![play_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
