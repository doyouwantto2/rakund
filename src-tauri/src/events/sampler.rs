use crate::engine::decoder;
use crate::setup::sound;
use serde::Deserialize;
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};

#[derive(Debug, Deserialize, Clone)]
pub struct KeyRange {
    pub low: u8,
    pub high: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SampleInfo {
    pub file: String,
    #[serde(rename = "keyRange")]
    pub key_range: KeyRange,
    #[serde(rename = "velocityRange")]
    pub velocity_range: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeyData {
    #[serde(rename = "midiNote")]
    pub midi_note: u8,
    #[serde(rename = "noteName")]
    pub note_name: String,
    pub samples: Vec<SampleInfo>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SplendidConfig {
    pub keys: HashMap<String, KeyData>,
}

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

    let target_layer = if velocity < 64 { "PP" } else { "FF" };
    let sample_info = key_data
        .samples
        .iter()
        .find(|s| s.file.contains(target_layer))
        .unwrap_or(&key_data.samples[0]);

    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Could not resolve resource dir: {}", e))?;

    // 3. Smart Path Resolution
    let final_path = if cfg!(dev) {
        // In development, look relative to the 'src-tauri' directory
        app.path()
            .app_config_dir() // This usually helps anchor us
            .unwrap()
            .parent()
            .unwrap() // Go up to project root
            .join("src-tauri")
            .join("data")
            .join("splendid")
            .join("Samples")
            .join(&sample_info.file)
    } else {
        app.path()
            .resource_dir()
            .map_err(|e| e.to_string())?
            .join("data")
            .join("splendid")
            .join("Samples")
            .join(&sample_info.file)
    };

    let final_path = if !final_path.exists() {
        std::env::current_dir()
            .unwrap()
            .join("data")
            .join("splendid")
            .join("Samples")
            .join(&sample_info.file)
    } else {
        final_path
    };
    println!("🎹 Attempting: {:?}", final_path);

    if !final_path.exists() {
        let err = format!("❌ FILE MISSING at: {:?}", final_path);
        eprintln!("{}", err);
        return Err(err);
    }

    let path_str = final_path.to_str().ok_or("Invalid path encoding")?;
    let data = decoder::decode_flac(path_str)?;

    let pitch_ratio = 2.0f32.powf((midi_num as f32 - key_data.midi_note as f32) / 12.0);

    if let Ok(mut voices_guard) = handle.active_voices.lock() {
        voices_guard.push(sound::Voice {
            data,
            playhead: 0.0,
            pitch_ratio,
            midi_note: midi_num,
            is_releasing: false,
            volume: (velocity as f32 / 127.0),
        });
        println!("✅ Voice active. Total: {}", voices_guard.len());
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
