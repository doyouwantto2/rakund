use crate::commands;
use crate::error::AudioError;
use crate::setup::sound;

pub fn run() -> Result<(), AudioError> {
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
