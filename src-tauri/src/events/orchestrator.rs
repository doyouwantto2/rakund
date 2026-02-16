use crate::engine::extractor;
use crate::models::SplendidConfig;
use crate::setup::sound;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn play_midi_note(
    midi_num: u8,
    velocity: u8,
    layer: String,
    handle: State<'_, sound::AudioHandle>,
    config: State<'_, SplendidConfig>,
    app: AppHandle,
) -> Result<(), String> {
    let key_data = config
        .keys
        .get(&midi_num.to_string())
        .ok_or_else(|| format!("Note {} not found", midi_num))?;

    let target_layer = if layer == "Auto" {
        if velocity < 50 {
            "PP"
        } else if velocity < 90 {
            "Mf"
        } else {
            "FF"
        }
    } else {
        &layer
    };

    let details = extractor::get_note_details(midi_num, key_data, target_layer, &app)?;

    let filename = details.final_path.file_name().unwrap().to_str().unwrap();

    let data = config
        .samples_cache
        .get(filename)
        .ok_or_else(|| format!("Sample {} not in RAM cache", filename))?
        .clone();

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
    println!("Sustain is now: {}", active);
    Ok(())
}
