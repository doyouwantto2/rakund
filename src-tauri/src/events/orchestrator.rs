use crate::engine::decoder;
use crate::models::SplendidConfig;
use crate::setup::sound;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn play_midi_note(
    midi_num: u8,
    velocity: u8,
    handle: State<'_, sound::AudioHandle>,
    config: State<'_, SplendidConfig>,
    _app: AppHandle,
) -> Result<(), String> {
    let key_data = config
        .keys
        .get(&midi_num.to_string())
        .ok_or_else(|| format!("Note {} not found in piano configuration", midi_num))?;

    // Find the appropriate sample based on velocity
    let sample_info = key_data
        .samples
        .iter()
        .find(|s| velocity >= s.min_vel && velocity <= s.max_vel)
        .or_else(|| key_data.samples.first())
        .ok_or_else(|| format!("No samples found for note {}", midi_num))?;

    // Calculate pitch ratio using the midi_note from key_data
    let pitch_ratio = 2.0f32.powf((midi_num as f32 - key_data.midi_note as f32) / 12.0);

    // Build the sample path
    let sample_path = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("data/splendid/Samples")
        .join(&sample_info.file);

    let path_str = sample_path
        .to_str()
        .ok_or_else(|| "Invalid character in sample file path".to_string())?;

    if !sample_path.exists() {
        return Err(format!(
            "Sample file not found: {}\nSample file: {}\nNote: {} ({})\nPlease ensure that 'splendid' folder exists in src-tauri/data/",
            path_str,
            sample_info.file,
            midi_num,
            key_data.note
        ));
    }

    let data = decoder::decode_flac(path_str)?;

    if let Ok(mut voices_guard) = handle.active_voices.lock() {
        let voices: &mut Vec<sound::Voice> = &mut *voices_guard;
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
pub async fn stop_midi_note(
    midi_num: u8,
    handle: State<'_, sound::AudioHandle>,
) -> Result<(), String> {
    let midi_num = midi_num;

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
