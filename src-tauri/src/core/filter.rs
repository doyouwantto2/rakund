use crate::setup::audio;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn set_sustain(
    active: bool,
    handle: State<'_, audio::AudioHandle>,
) -> Result<(), String> {
    if let Ok(mut sustain) = handle.is_sustained.lock() {
        *sustain = active;
    }
    if !active {
        if let Ok(mut voices) = handle.active_voices.lock() {
            for v in voices.iter_mut() {
                if v.is_releasing {}
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_midi_note(
    midi_num: u8,
    handle: State<'_, audio::AudioHandle>,
    _app: AppHandle,
) -> Result<(), String> {
    if let Ok(mut voices) = handle.active_voices.lock() {
        for v in voices.iter_mut().filter(|v| v.midi_note == midi_num) {
            v.is_releasing = true;
        }
    }
    Ok(())
}
