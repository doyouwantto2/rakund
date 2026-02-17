use crate::engine::{basis, decoder};
use crate::setup::sound::AudioHandle;
use crate::setup::sound::Voice;
use std::sync::{Arc, Mutex};
use tauri::State;

pub fn play_sound(active_voices: &Arc<Mutex<Vec<Voice>>>, data: Arc<Vec<f32>>) {
    if let Ok(mut voices) = active_voices.lock() {
        voices.push(Voice {
            data,
            playhead: 0.0,
            pitch_ratio: 1.0,
            midi_note: 0,
            is_releasing: false,
            volume: 1.0,
        });
    }
}

pub async fn play_note(path: String, handle: State<'_, AudioHandle>) -> Result<(), String> {
    let sound_data = decoder::decode_flac(&path)
        .map_err(|e| e.to_string())?;
    basis::play_sound(&handle.active_voices, sound_data);
    Ok(())
}
