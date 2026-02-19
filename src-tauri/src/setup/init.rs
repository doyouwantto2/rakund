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

    // Read last instrument name BEFORE building the app — just the name, no loading yet
    let last_instrument = state::read().ok().and_then(|s| s.last_instrument);

    tauri::Builder::default()
        .manage(audio_handle)
        .invoke_handler(tauri::generate_handler![
            commands::player::play_midi_note,
            commands::player::load_instrument,
            commands::player::load_instrument_layer,
            commands::player::get_available_instruments,
            commands::player::get_instrument_info,
            commands::player::get_app_state,
            commands::filter::stop_midi_note,
            commands::filter::set_sustain,
        ])
        .setup(move |app| {
            // Window is now open. Kick off background preload if we have a last instrument.
            if let Some(folder) = last_instrument {
                println!("[INIT] Scheduling background preload: {}", folder);

                let app_handle = app.handle().clone();

                std::thread::spawn(move || {
                    println!("[INIT] Background preload starting: {}", folder);

                    // Emit: loading started
                    let _ = app_handle.emit("load_progress",
                        serde_json::json!({ "progress": 0.0, "status": "loading" }));

                    match audio::load_instrument_with_progress(&folder, &app_handle) {
                        Ok(config) => {
                            *crate::commands::player::CURRENT_INSTRUMENT
                                .lock()
                                .unwrap() = Some(config);

                            println!("[INIT] Background preload complete: {}", folder);

                            // Emit: done
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
