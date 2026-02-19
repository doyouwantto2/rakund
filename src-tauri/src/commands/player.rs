use crate::engine::cache;
use crate::error::AudioError;
use crate::setup::audio::{self, AudioHandle};
use crate::setup::models::{AppState, InstrumentConfig, InstrumentInfo};
use crate::setup::state;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State};

lazy_static! {
    pub static ref CURRENT_INSTRUMENT: Arc<Mutex<Option<InstrumentConfig>>> =
        Arc::new(Mutex::new(None));
}

// ── Playback ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn play_midi_note(
    midi_num: u8,
    velocity: u8,
    layer: String, // uppercased layer name from frontend e.g. "MP"
    handle: State<'_, AudioHandle>,
    _app: AppHandle,
) -> Result<(), String> {
    let config_guard = CURRENT_INSTRUMENT.lock().unwrap();
    let config = config_guard.as_ref().ok_or("No instrument loaded")?;

    let key_data = config
        .piano_keys
        .get(&midi_num.to_string())
        .ok_or_else(|| AudioError::NoteNotFound(midi_num).to_string())?;

    // Find the layer index by matching uppercased names in general.layers.
    // Frontend sends uppercase (e.g. "MP"), general.layers may have any case ("Mp", "MP").
    // We match uppercase → uppercase so case in JSON doesn't matter.
    let layer_idx = config
        .layers()
        .iter()
        .position(|l| l.to_uppercase() == layer.to_uppercase())
        .unwrap_or(0); // fall back to first layer if not found

    // Look up by index — completely independent of name/case
    let data = cache::get_by_index(midi_num, layer_idx).ok_or_else(|| {
        format!(
            "Sample not cached: midi={} layer_idx={}",
            midi_num, layer_idx
        )
    })?;

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

// ── Instrument management ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_available_instruments() -> Result<Vec<InstrumentInfo>, String> {
    let folders = audio::scan_instruments().map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for folder in folders {
        let json_path = folder.join("instrument.json");
        let raw = std::fs::read_to_string(&json_path).unwrap_or_default();
        if let Ok(config) = serde_json::from_str::<InstrumentConfig>(&raw) {
            result.push(InstrumentInfo {
                name: config.instrument.clone(),
                folder: folder
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                // Always send uppercase layer names to frontend
                layers: config.layers().iter().map(|l| l.to_uppercase()).collect(),
                format: config.files_format().to_string(),
            });
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn load_instrument(folder: String) -> Result<InstrumentInfo, String> {
    let config = audio::load_instrument(&folder).map_err(|e| e.to_string())?;

    state::set_last_instrument(&folder).map_err(|e: AudioError| e.to_string())?;

    let info = InstrumentInfo {
        name: config.instrument.clone(),
        folder,
        layers: config.layers().iter().map(|l| l.to_uppercase()).collect(),
        format: config.files_format().to_string(),
    };

    *CURRENT_INSTRUMENT.lock().unwrap() = Some(config);
    Ok(info)
}

#[tauri::command]
pub async fn get_app_state() -> Result<AppState, String> {
    state::read().map_err(|e: AudioError| e.to_string())
}

#[tauri::command]
pub async fn get_instrument_info() -> Result<Option<InstrumentInfo>, String> {
    let guard = CURRENT_INSTRUMENT.lock().unwrap();
    Ok(guard.as_ref().map(|config| InstrumentInfo {
        name: config.instrument.clone(),
        folder: String::new(),
        layers: config.layers().iter().map(|l| l.to_uppercase()).collect(),
        format: config.files_format().to_string(),
    }))
}

#[tauri::command]
pub async fn load_instrument_layer(_layer: String, window: tauri::Window) -> Result<(), String> {
    let _ = window.emit(
        "load_progress",
        serde_json::json!({ "loaded": 1, "total": 1, "progress": 100.0 }),
    );
    Ok(())
}
