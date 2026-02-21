use crate::engine::cache;
use crate::error::AudioError;
use crate::extension::instrument::response::InstrumentInfoResponse;
use crate::setup::audio::{self, AudioHandle};
use crate::setup::config::{AppState, InstrumentConfig};
use crate::setup::state;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};

lazy_static! {
    pub static ref CURRENT_INSTRUMENT: Arc<Mutex<Option<InstrumentConfig>>> =
        Arc::new(Mutex::new(None));
    pub static ref CURRENT_FOLDER: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

#[tauri::command]
pub async fn play_midi_note(
    midi_num: u8,
    velocity: u8,
    layer: String,
    handle: State<'_, AudioHandle>,
    _app: AppHandle,
) -> Result<(), String> {
    let config_guard = CURRENT_INSTRUMENT.lock().unwrap();
    let config = config_guard.as_ref().ok_or("No instrument loaded")?;

    let key_data = config
        .piano_keys
        .get(&midi_num.to_string())
        .ok_or_else(|| AudioError::NoteNotFound(midi_num).to_string())?;

    let layer_upper = layer.to_uppercase();
    let _sample_info = key_data
        .samples
        .iter()
        .find(|s| s.layer.to_uppercase() == layer_upper)
        .or_else(|| key_data.samples.first())
        .ok_or_else(|| format!("No samples for note {}", midi_num))?;

    let sample_idx = key_data
        .samples
        .iter()
        .position(|s| s.layer.to_uppercase() == layer_upper)
        .unwrap_or(0);

    let data = cache::get_by_index(midi_num, sample_idx)
        .ok_or_else(|| format!("Sample not cached: midi={} layer={}", midi_num, layer))?;

    let recorded_midi = audio::pitch_to_midi(&key_data.pitch).unwrap_or(key_data.midi_num());
    let ratio = audio::pitch_ratio(recorded_midi, midi_num);

    if let Ok(mut voices) = handle.active_voices.lock() {
        voices.push(crate::setup::audio::Voice {
            data,
            playhead: 0.0,
            pitch_ratio: ratio,
            midi_note: midi_num,
            is_releasing: false,
            volume: velocity as f32 / 127.0,
        });
    }

    Ok(())
}

#[tauri::command]
pub async fn get_available_instruments() -> Result<Vec<InstrumentInfoResponse>, String> {
    let folders = audio::scan_instruments().map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for folder in folders {
        let json_path = folder.join("instrument.json");
        println!("[SCAN] Checking instrument at: {:?}", json_path);

        let raw = std::fs::read_to_string(&json_path).unwrap_or_default();
        if raw.is_empty() {
            println!("[SCAN] Empty JSON file at: {:?}", json_path);
            continue;
        }

        match serde_json::from_str::<InstrumentConfig>(&raw) {
            Ok(config) => {
                let folder_name = folder
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let info = InstrumentInfoResponse::from_config(&config, &folder_name);
                result.push(info);
            }
            Err(e) => {
                println!("[SCAN] Failed to parse JSON at {:?}: {}", json_path, e);
                println!(
                    "[SCAN] JSON content preview: {}",
                    &raw[..std::cmp::min(200, raw.len())]
                );
            }
        }
    }

    println!("[SCAN] Found {} valid instruments", result.len());
    Ok(result)
}

#[tauri::command]
pub async fn load_instrument(
    folder: String,
    app: AppHandle,
) -> Result<InstrumentInfoResponse, String> {
    {
        let current = CURRENT_FOLDER.lock().unwrap();
        if current.as_deref() == Some(folder.as_str()) {
            let config_guard = CURRENT_INSTRUMENT.lock().unwrap();
            if let Some(config) = config_guard.as_ref() {
                return Ok(InstrumentInfoResponse::from_config(config, &folder));
            }
        }
    }

    let info = audio::load_instrument_with_progress(&folder, &app)
        .map_err(|e| e.to_string())
        .map(|config| {
            let info = InstrumentInfoResponse::from_config(&config, &folder);
            *CURRENT_INSTRUMENT.lock().unwrap() = Some(config);
            *CURRENT_FOLDER.lock().unwrap() = Some(folder.clone());
            info
        })?;

    state::set_last_instrument(&folder).map_err(|e: AudioError| e.to_string())?;

    Ok(info)
}

#[tauri::command]
pub async fn get_app_state() -> Result<AppState, String> {
    state::read().map_err(|e: AudioError| e.to_string())
}

#[tauri::command]
pub async fn get_instrument_info() -> Result<Option<InstrumentInfoResponse>, String> {
    let config_guard = CURRENT_INSTRUMENT.lock().unwrap();
    let folder_guard = CURRENT_FOLDER.lock().unwrap();

    Ok(config_guard.as_ref().map(|config| {
        let folder_name = folder_guard.clone().unwrap_or_default();
        InstrumentInfoResponse::from_config(config, &folder_name)
    }))
}

#[tauri::command]
pub async fn clear_last_instrument() -> Result<(), String> {
    state::clear_last_instrument().map_err(|e: AudioError| e.to_string())
}

pub fn set_current_folder(folder: String) {
    *CURRENT_FOLDER.lock().unwrap() = Some(folder);
}
