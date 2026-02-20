use crate::commands;
use crate::error::AudioError;
use crate::setup::{audio, state};
use tauri::Emitter;

pub fn run() -> Result<(), AudioError> {
    let audio_handle = audio::start_stream()?;

    match state::instruments_dir() {
        Ok(dir) => println!("[INIT] Instruments directory: {:?}", dir),
        Err(e) => eprintln!("[INIT] Cannot resolve instruments dir: {}", e),
    }

    let last_instrument = state::read().ok().and_then(|s| s.last_instrument);

    tauri::Builder::default()
        .manage(audio_handle)
        .invoke_handler(tauri::generate_handler![
            commands::player::play_midi_note,
            commands::player::load_instrument,
            commands::player::get_available_instruments,
            commands::player::get_instrument_info,
            commands::player::get_app_state,
            commands::filter::stop_midi_note,
            commands::filter::set_sustain,
        ])
        .setup(move |app| {
            if let Some(folder) = last_instrument {
                println!("[INIT] Scheduling background preload: {}", folder);
                let app_handle = app.handle().clone();
                let folder_clone = folder.clone();

                std::thread::spawn(move || {
                    println!("[INIT] Background preload starting: {}", folder_clone);

                    let _ = app_handle.emit("load_progress",
                        serde_json::json!({ "progress": 0.0, "status": "loading" }));

                    match audio::load_instrument_with_progress(&folder_clone, &app_handle) {
                        Ok(config) => {
                            // Store the config AND the folder name — both must be set atomically
                            *crate::commands::player::CURRENT_INSTRUMENT.lock().unwrap() = Some(config);
                            crate::commands::player::set_current_folder(folder_clone.clone());

                            println!("[INIT] Background preload complete: {}", folder_clone);

                            let _ = app_handle.emit("load_progress",
                                serde_json::json!({ "progress": 100.0, "status": "done" }));
                        }
                        Err(e) => {
                            eprintln!("[INIT] Background preload failed: {}", e);
                            let _ = app_handle.emit("load_progress",
                                serde_json::json!({ "progress": -1.0, "status": "error", "message": e.to_string() }));
                        }
                    }
                });
            } else {
                println!("[INIT] No last instrument — skipping preload");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
