use crate::setup::sound;
use crate::setup::models::create_instrument;
use crate::instrument::{load_splendid_layer, load_salamander_layer, get_splendid_sample, get_salamander_sample, is_splendid_layer_loaded, is_salamander_layer_loaded};
use crate::error::AudioError;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn play_midi_note(
    midi_num: u8,
    velocity: u8,
    instrument: String,
    layer: String,
    handle: State<'_, sound::AudioHandle>,
    _app: AppHandle,
) -> Result<(), String> {
    println!("[DEBUG] Playing note {} with instrument {} and layer {}", midi_num, instrument, layer);
    
    // Create instrument instance
    let instrument_instance = create_instrument(&instrument)
        .map_err(|e| e.to_string())?;
    
    let key_data = instrument_instance.config()
        .keys
        .get(&midi_num.to_string())
        .ok_or_else(|| AudioError::NoteNotFound(midi_num).to_string())?;

    println!("[DEBUG] Available samples for note {}: {:?}", midi_num, 
        key_data.samples.iter().map(|s| (&s.layer, &s.file)).collect::<Vec<_>>());

    let sample_info = key_data
        .samples
        .iter()
        .find(|s| {
            println!("[DEBUG] Comparing layer '{}' with sample.layer '{}' (velocity: {})", layer, s.layer, velocity);
            layer == s.layer && velocity >= s.min_vel && velocity <= s.max_vel
        })
        .or_else(|| {
            println!("[DEBUG] Layer {} not found, trying first sample", layer);
            key_data.samples.first()
        })
        .ok_or_else(|| AudioError::NoSamplesFound(midi_num).to_string())?;

    println!("[DEBUG] Selected sample: {} for layer {} (velocity: {})", sample_info.file, sample_info.layer, velocity);

    let pitch_ratio = 2.0f32.powf((midi_num as f32 - key_data.midi_note as f32) / 12.0);

    let data = match instrument.as_str() {
        "splendid" => {
            println!("[DEBUG] Looking for splendid sample: {}", sample_info.file);
            get_splendid_sample(&sample_info.file)
        },
        "salamander" => {
            println!("[DEBUG] Looking for salamander sample: {}", sample_info.file);
            get_salamander_sample(&sample_info.file)
        },
        _ => return Err("Unknown instrument".to_string()),
    }.ok_or_else(|| {
        println!("[DEBUG] Sample {} not found in cache", sample_info.file);
        format!("Sample {} not found in cache", sample_info.file)
    })?;

    println!("[DEBUG] Found sample data with {} samples", data.len());

    if let Ok(mut voices_guard) = handle.active_voices.lock() {
        let voices: &mut Vec<sound::Voice> = &mut voices_guard;
        voices.push(sound::Voice {
            data,
            playhead: 0.0,
            pitch_ratio,
            midi_note: midi_num,
            is_releasing: false,
            volume: (velocity as f32 / 127.0),
        });
    }

    Ok(())
}

#[tauri::command]
pub async fn load_instrument_layer(
    instrument: String,
    layer: String,
) -> std::result::Result<(), String> {
    let instrument_instance = create_instrument(&instrument)
        .map_err(|e| e.to_string())?;
    
    match instrument.as_str() {
        "splendid" => {
            let result: std::result::Result<(), crate::error::AudioError> = load_splendid_layer(&layer, instrument_instance.config()).await;
            result.map_err(|e| e.to_string())
        },
        "salamander" => {
            let result: std::result::Result<(), crate::error::AudioError> = load_salamander_layer(&layer, instrument_instance.config()).await;
            result.map_err(|e| e.to_string())
        },
        _ => Err("Unknown instrument".to_string()),
    }
}

#[tauri::command]
pub async fn is_layer_loaded(
    instrument: String,
    layer: String,
) -> std::result::Result<bool, String> {
    match instrument.as_str() {
        "splendid" => Ok(is_splendid_layer_loaded(&layer)),
        "salamander" => Ok(is_salamander_layer_loaded(&layer)),
        _ => Err("Unknown instrument".to_string()),
    }
}

#[tauri::command]
pub async fn get_instrument_info(instrument: String) -> std::result::Result<crate::setup::models::InstrumentInfo, String> {
    let instrument_instance = create_instrument(&instrument)
        .map_err(|e| e.to_string())?;
    
    Ok(crate::setup::models::InstrumentInfo {
        name: instrument_instance.name().to_string(),
        layers: instrument_instance.get_layers(),
    })
}
