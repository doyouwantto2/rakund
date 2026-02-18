use crate::error::AudioError;
use crate::setup::models::InstrumentConfig;
use crate::setup::sound;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

lazy_static! {
    pub static ref INSTRUMENT_CONFIG: Arc<std::sync::Mutex<Option<InstrumentConfig>>> =
        Arc::new(std::sync::Mutex::new(None));

    /// Keyed as "{midi}:{layer}" — e.g. "60:PP", "60:MP", "61:PP"
    /// Each note+layer is its own entry, so no two notes ever share a lookup.
    pub static ref SAMPLE_CACHE: Arc<std::sync::Mutex<HashMap<String, Arc<Vec<f32>>>>> =
        Arc::new(std::sync::Mutex::new(HashMap::new()));
}

fn pitch_ratio(recorded_midi: u8, target_midi: u8) -> f32 {
    2.0f32.powf((target_midi as f32 - recorded_midi as f32) / 12.0)
}

#[tauri::command]
pub async fn play_midi_note(
    midi_num: u8,
    velocity: u8,
    layer: String,
    handle: State<'_, sound::AudioHandle>,
    _app: AppHandle,
) -> Result<(), String> {
    // Lazy-load config
    {
        let mut cfg = INSTRUMENT_CONFIG.lock().unwrap();
        if cfg.is_none() {
            *cfg = Some(load_instrument_config().map_err(|e| e.to_string())?);
        }
    }
    let config = INSTRUMENT_CONFIG.lock().unwrap().as_ref().unwrap().clone();

    let key_data = config
        .keys
        .get(&midi_num.to_string())
        .ok_or_else(|| AudioError::NoteNotFound(midi_num).to_string())?;

    // Find the sample for the requested layer (fall back to first)
    let sample_info = key_data
        .samples
        .iter()
        .find(|s| s.layer == layer)
        .or_else(|| key_data.samples.first())
        .ok_or_else(|| format!("No samples for note {}", midi_num))?;

    let actual_layer = &sample_info.layer;

    // Exact lookup — "{midi}:{layer}"
    let cache_key = format!("{}:{}", midi_num, actual_layer);
    let data = SAMPLE_CACHE
        .lock()
        .unwrap()
        .get(&cache_key)
        .cloned()
        .ok_or_else(|| {
            format!(
                "Sample not cached for midi={} layer={}. Is the layer loaded?",
                midi_num, actual_layer
            )
        })?;

    // Pitch ratio: recorded at key_data.midi_note, playing at midi_num
    let ratio = pitch_ratio(key_data.midi_note, midi_num);

    if let Ok(mut voices) = handle.active_voices.lock() {
        voices.push(sound::Voice {
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
pub async fn load_instrument_layer(layer: String, window: tauri::Window) -> Result<(), String> {
    // All layers are loaded at startup by initialize_audio().
    // This command exists for frontend compatibility — emit 100% immediately.
    let _ = window.emit(
        "load_progress",
        serde_json::json!({
            "layer": layer,
            "loaded": 1,
            "total": 1,
            "progress": 100.0
        }),
    );
    Ok(())
}

#[tauri::command]
pub async fn get_instrument_info() -> Result<serde_json::Value, String> {
    let mut cfg = INSTRUMENT_CONFIG.lock().unwrap();
    if cfg.is_none() {
        *cfg = Some(load_instrument_config().map_err(|e| e.to_string())?);
    }
    let config = cfg.as_ref().unwrap();
    Ok(serde_json::json!({
        "name": config.instrument,
        "layers": config.layers
    }))
}

fn load_instrument_config() -> Result<InstrumentConfig, AudioError> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let path = cwd.join("data").join("instrument.json");
    let raw = fs::read_to_string(&path)
        .map_err(|e| AudioError::InstrumentError(format!("Cannot read instrument.json: {}", e)))?;
    serde_json::from_str(&raw)
        .map_err(|e| AudioError::InstrumentError(format!("Invalid JSON: {}", e)))
}
