use crate::models::KeyData;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub struct ExtractedNote {
    pub final_path: PathBuf,
    pub pitch_ratio: f32,
}

pub fn get_note_details(
    midi_num: u8,
    key_data: &KeyData,
    target_layer: &str,
    app: &AppHandle,
) -> Result<ExtractedNote, String> {
    let search_term = target_layer.to_lowercase();

    let sample_info = key_data
        .samples
        .iter()
        .find(|s| s.file.to_lowercase().contains(&search_term)) // Case-insensitive match
        .unwrap_or_else(|| {
            key_data
                .samples
                .iter()
                .find(|s| s.file.contains("FF"))
                .unwrap_or(&key_data.samples[0])
        });

    let pitch_ratio = 2.0f32.powf((midi_num as f32 - key_data.midi_note as f32) / 12.0);

    let path = app
        .path()
        .app_config_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("src-tauri")
        .join("data")
        .join("splendid")
        .join("Samples")
        .join(&sample_info.file);

    Ok(ExtractedNote {
        final_path: path,
        pitch_ratio,
    })
}
