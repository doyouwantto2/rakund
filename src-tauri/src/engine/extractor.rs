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
    let search = target_layer.to_lowercase();

    println!(
        "--- 🎹 MIDI Trigger: {} | Layer Search: {} ---",
        midi_num, search
    );

    let sample_info = key_data
        .samples
        .iter()
        .find(|s| {
            let found = s.layer.to_lowercase() == search;
            if found {
                println!("🎯 Match Found: Layer '{}'", s.layer);
            }
            found
        })
        .or_else(|| {
            let ff_fallback = key_data.samples.iter().find(|s| s.layer == "FF");
            if ff_fallback.is_some() {
                println!("⚠️ Fallback: Using 'FF' layer");
            }
            ff_fallback
        })
        .unwrap_or_else(|| {
            println!("🛑 Default: No match, using first sample in list");
            &key_data.samples[0]
        });

    let semitone_distance = midi_num as f32 - key_data.midi_note as f32;
    let pitch_ratio = 2.0f32.powf(semitone_distance / 12.0);

    println!(
        "⚖️ Pitch Math: {} (Target) - {} (Root) = {} semitones",
        midi_num, key_data.midi_note, semitone_distance
    );
    println!("📈 Calculated Ratio: {:.4}", pitch_ratio);

    let path = app
        .path()
        .resolve(
            "data/splendid/Samples",
            tauri::path::BaseDirectory::Resource,
        )
        .map_err(|e| format!("Resource path error: {}", e))?
        .join(&sample_info.file);

    println!("📂 Final File Path: {:?}", path);

    if !path.exists() {
        println!("❌ FILE NOT FOUND ON DISK!");
        return Err(format!("File not found: {:?}", path));
    } else {
        println!("✅ FILE OK");
    }

    Ok(ExtractedNote {
        final_path: path,
        pitch_ratio,
    })
}
