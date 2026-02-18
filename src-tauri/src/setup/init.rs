use crate::commands;
use crate::setup::sound;
use crate::error::AudioError;

pub fn run() -> Result<(), AudioError> {
    // Initialize audio and load all FLAC files into RAM
    sound::initialize_audio()?;
    
    let audio_handle = sound::start_stream()?;

    println!("🎹 Multi-instrument system enabled. Instruments will be loaded dynamically.");

    tauri::Builder::default()
        .manage(audio_handle)
        .invoke_handler(tauri::generate_handler![
            commands::player::play_midi_note,
            commands::filter::stop_midi_note,
            commands::filter::set_sustain,
            commands::player::load_instrument_layer,
            commands::player::get_instrument_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
