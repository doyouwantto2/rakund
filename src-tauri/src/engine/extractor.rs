use crate::models::KeyData;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub struct ExtractedNote {
    pub final_path: PathBuf,
    pub pitch_ratio: f32,
}

pub fn get_note_details(
    midi_num: u8,
    velocity: u8,
    key_data: &KeyData,
    app: &AppHandle,
) -> Result<ExtractedNote, String> {
    // 1. Selection logic
    let target_layer = if velocity < 64 { "PP" } else { "FF" };
    let sample_info = key_data
        .samples
        .iter()
        .find(|s| s.file.contains(target_layer))
        .unwrap_or(&key_data.samples[0]);

    // 2. Pitch logic
    let pitch_ratio = 2.0f32.powf((midi_num as f32 - key_data.midi_note as f32) / 12.0);

    // 3. Ultra-Robust Path Finding
    let filename = &sample_info.file;

    // Attempt 1: Standard Resource Dir (Production)
    let res_dir = app.path().resource_dir().ok();

    // Attempt 2: Local Data (Nix/Dev Shell)
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    let potential_paths = [
        // Path A: The bundled location
        res_dir
            .clone()
            .map(|d| d.join("data/splendid/Samples").join(filename)),
        // Path B: The dev location (running from project root)
        Some(cwd.join("src-tauri/data/splendid/Samples").join(filename)),
        // Path C: Direct data access (running from src-tauri)
        Some(cwd.join("data/splendid/Samples").join(filename)),
    ];

    let mut final_path = None;
    for path in potential_paths.into_iter().flatten() {
        if path.exists() {
            final_path = Some(path);
            break;
        }
    }

    let valid_path = final_path
        .ok_or_else(|| format!("❌ Sample not found: {}\nChecked CWD: {:?}", filename, cwd))?;

    Ok(ExtractedNote {
        final_path: valid_path,
        pitch_ratio,
    })
}
