use crate::engine::decoder;
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
    pub static ref SAMPLE_CACHE: Arc<std::sync::Mutex<HashMap<String, Arc<Vec<f32>>>>> =
        Arc::new(std::sync::Mutex::new(HashMap::new()));
}

#[tauri::command]
pub async fn play_midi_note(
    midi_num: u8,
    velocity: u8,
    layer: String,
    handle: State<'_, sound::AudioHandle>,
    _app: AppHandle,
) -> Result<(), String> {
    // Load instrument config if not loaded
    {
        let mut config = INSTRUMENT_CONFIG.lock().unwrap();
        if config.is_none() {
            *config = Some(load_instrument_config().map_err(|e| e.to_string())?);
        }
    }

    let config = INSTRUMENT_CONFIG.lock().unwrap().as_ref().unwrap().clone();

    let key_data = config
        .keys
        .get(&midi_num.to_string())
        .ok_or_else(|| AudioError::NoteNotFound(midi_num).to_string())?;

    let sample_info = key_data
        .samples
        .iter()
        .find(|s| layer == s.layer)
        .or_else(|| key_data.samples.first())
        .ok_or_else(|| format!("No samples found for note {}", midi_num))?;

    // Try to get from cache first
    let cache = SAMPLE_CACHE.lock().unwrap();

    // Try exact match first
    if let Some(cached_data) = cache.get(&sample_info.file).cloned() {
        let data = cached_data;

        if let Ok(mut voices_guard) = handle.active_voices.lock() {
            let voices: &mut Vec<sound::Voice> = &mut voices_guard;
            voices.push(sound::Voice {
                data: data.clone(),
                playhead: 0.0,
                pitch_ratio: 1.0,
                midi_note: midi_num,
                is_releasing: false,
                volume: (velocity as f32 / 127.0),
            });
        }
        return Ok(());
    }

    // Try case-insensitive match
    let target_name_lower = sample_info.file.to_lowercase();
    for (cached_name, cached_data) in cache.iter() {
        if cached_name.to_lowercase() == target_name_lower {
            let data = cached_data.clone();

            if let Ok(mut voices_guard) = handle.active_voices.lock() {
                let voices: &mut Vec<sound::Voice> = &mut voices_guard;
                voices.push(sound::Voice {
                    data: data.clone(),
                    playhead: 0.0,
                    pitch_ratio: 1.0,
                    midi_note: midi_num,
                    is_releasing: false,
                    volume: (velocity as f32 / 127.0),
                });
            }
            return Ok(());
        }
    }

    // Try partial match (layer name)
    for (cached_name, cached_data) in cache.iter() {
        if cached_name
            .to_lowercase()
            .contains(&sample_info.layer.to_lowercase())
        {
            let data = cached_data.clone();

            if let Ok(mut voices_guard) = handle.active_voices.lock() {
                let voices: &mut Vec<sound::Voice> = &mut voices_guard;
                voices.push(sound::Voice {
                    data: data.clone(),
                    playhead: 0.0,
                    pitch_ratio: 1.0,
                    midi_note: midi_num,
                    is_releasing: false,
                    volume: (velocity as f32 / 127.0),
                });
            }
            return Ok(());
        }
    }

    return Err(format!("Sample {} not found in cache", sample_info.file));
}

#[tauri::command]
pub async fn load_instrument_layer(layer: String, window: tauri::Window) -> Result<(), String> {
    let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    let samples_dir = current_dir.join("data/instrument/Samples");

    if !samples_dir.exists() {
        return Err("Samples directory not found".to_string());
    }

    let mut total_files = 0;
    let mut loaded_count = 0;

    // Count total FLAC files first
    for entry in std::fs::read_dir(&samples_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("flac") {
            total_files += 1;
        }
    }

    // Load all FLAC files, skipping any already in cache
    for entry in std::fs::read_dir(&samples_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("flac") {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            // Skip if already cached from initialize_audio()
            {
                let cache = SAMPLE_CACHE.lock().unwrap();
                if cache.contains_key(&file_name) {
                    loaded_count += 1;
                    continue;
                }
            }

            println!(
                "[LOADING] [{}/{}] Processing: {}",
                loaded_count + 1,
                total_files,
                file_name
            );

            let file_path_str = path.to_string_lossy().to_string();
            let decoded = decoder::decode_flac(&file_path_str)
                .map_err(|e| format!("Failed to decode {}: {}", file_name, e))?;

            let mut cache = SAMPLE_CACHE.lock().unwrap();
            cache.insert(file_name.clone(), decoded);
            loaded_count += 1;

            // Emit progress event
            let progress = (loaded_count as f32 / total_files as f32) * 100.0;
            let event_data = serde_json::json!({
                "layer": layer,
                "loaded": loaded_count,
                "total": total_files,
                "progress": progress
            });

            println!(
                "[PROGRESS] [{}/{}] {:.1}% complete - {}",
                loaded_count, total_files, progress, file_name
            );

            match window.emit("load_progress", event_data) {
                Ok(_) => {}
                Err(e) => println!("[ERROR] Failed to emit progress event: {:?}", e),
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_instrument_info() -> Result<serde_json::Value, String> {
    let config = {
        let mut global_config = INSTRUMENT_CONFIG.lock().unwrap();
        if global_config.is_none() {
            *global_config = Some(load_instrument_config().map_err(|e| e.to_string())?);
        }
        global_config.take().unwrap()
    };

    Ok(serde_json::json!({
        "name": config.instrument,
        "layers": config.layers
    }))
}

fn load_instrument_config() -> Result<InstrumentConfig, AudioError> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let config_path = cwd.join("data").join("instrument.json");

    let config_data = fs::read_to_string(&config_path).map_err(|e| {
        AudioError::InstrumentError(format!("Could not find JSON at {:?}: {}", config_path, e))
    })?;

    let config: InstrumentConfig = serde_json::from_str(&config_data)
        .map_err(|e| AudioError::InstrumentError(format!("Invalid JSON: {}", e)))?;

    Ok(config)
}
