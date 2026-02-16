use crate::events::audio;
use crate::setup;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let audio_handle = setup::sound::start_stream();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(audio_handle)
        .invoke_handler(tauri::generate_handler![audio::play_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
