use crate::engine::{decoder, extractor};
use crate::models::SplendidConfig; // Import from models
use crate::setup::sound;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn play_midi_note(
    midi_num: u8,
    velocity: u8,
    handle: State<'_, sound::AudioHandle>,
    config: State<'_, SplendidConfig>,
    app: AppHandle,
) -> Result<(), String> {
    let key_data = config
        .keys
        .get(&midi_num.to_string())
        .ok_or_else(|| format!("Note {} not found", midi_num))?;

    let details = extractor::get_note_details(midi_num, velocity, key_data, &app)?;

    let path_str = details.final_path.to_str().ok_or("Invalid path")?;
    let data = decoder::decode_flac(path_str)?;

    // THE CRITICAL SECTION
    // We lock the mutex only for the push, keeping the lock time minimal
    if let Ok(mut voices_guard) = handle.active_voices.lock() {
        voices_guard.push(sound::Voice {
            data,
            playhead: 0.0,
            pitch_ratio: details.pitch_ratio,
            midi_note: midi_num,
            is_releasing: false,
            volume: (velocity as f32 / 127.0),
        });
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_midi_note(
    midi_num: u8,
    handle: State<'_, sound::AudioHandle>,
) -> Result<(), String> {
    if let Ok(mut voices) = handle.active_voices.lock() {
        for v in voices.iter_mut().filter(|v| v.midi_note == midi_num) {
            v.is_releasing = true;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn set_sustain(
    active: bool,
    handle: State<'_, sound::AudioHandle>,
) -> Result<(), String> {
    if let Ok(mut sustain) = handle.is_sustained.lock() {
        *sustain = active;
    }
    Ok(())
}
