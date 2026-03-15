use crate::setup::audio::AudioHandle;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn play_midi_note(
    _midi_num: u8,
    _velocity: u8,
    _layer: String,
    _handle: State<'_, AudioHandle>,
    _app: AppHandle,
) -> Result<(), String> {
    // For now, just return an error since we don't have instrument loaded
    return Err("No instrument loaded".to_string());
}

#[tauri::command]
pub async fn stop_midi_note(
    midi_num: u8,
    _layer: String,
    handle: State<'_, AudioHandle>,
) -> Result<(), String> {
    let mut active_voices = handle.active_voices.lock().unwrap();
    active_voices.retain(|v| !(v.midi_note == midi_num && v.is_releasing == false));
    for v in active_voices.iter_mut() {
        if v.midi_note == midi_num && v.is_releasing == false {
            v.is_releasing = true;
        }
    }

    Ok(())
}
