use crate::extra::challenge::loader::buffer::{MidiBuffer, MidiNoteMs};
use crate::extra::challenge::parser::midi::MidiParser;
use crate::setup::state;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VisualizerError {
    #[error("Failed to load MIDI file: {0}")]
    LoadError(#[from] crate::extra::challenge::parser::midi::MidiParseError),

    #[error("Invalid file path")]
    InvalidPath,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiSessionInfo {
    pub total_duration_ms: u32,
    pub tempo_bpm: f32,
    pub note_count: usize,
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongInfo {
    pub file_name: String,
    pub file_path: String,
    pub display_name: String,
}

lazy_static! {
    pub static ref CURRENT_BUFFER: Arc<Mutex<Option<MidiBuffer>>> = Arc::new(Mutex::new(None));
}

#[tauri::command]
pub fn scan_songs() -> Result<Vec<SongInfo>, String> {
    let dir = state::songs_dir().map_err(|e| e.to_string())?;

    let entries =
        std::fs::read_dir(&dir).map_err(|e| format!("Cannot read songs directory: {}", e))?;

    let songs: Vec<SongInfo> = entries
        .flatten()
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("mid") || ext.eq_ignore_ascii_case("midi"))
                .unwrap_or(false)
        })
        .map(|e| {
            let path = e.path();
            let file_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let display_name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .replace(['_', '-'], " ")
                .split_whitespace()
                .map(|w| {
                    let mut chars = w.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            SongInfo {
                file_name,
                file_path: path.to_string_lossy().to_string(),
                display_name,
            }
        })
        .collect();

    println!("[SONGS] Found {} MIDI files", songs.len());
    Ok(songs)
}

#[tauri::command]
pub fn load_midi_session(file_path: String) -> Result<MidiSessionInfo, String> {
    let midi_file = MidiParser::parse_file(&file_path)
        .map_err(|e| format!("Failed to parse MIDI file: {}", e))?;

    let buffer = MidiBuffer::from_midi_file(&midi_file, file_path.clone());

    let info = MidiSessionInfo {
        total_duration_ms: buffer.total_duration_ms,
        tempo_bpm: buffer.tempo_bpm,
        note_count: buffer.all_notes.len(),
        file_path: file_path.clone(),
    };

    *CURRENT_BUFFER.lock().unwrap() = Some(buffer);

    Ok(info)
}

#[tauri::command]
pub fn get_session_notes() -> Result<Vec<MidiNoteMs>, String> {
    let guard = CURRENT_BUFFER.lock().unwrap();
    match guard.as_ref() {
        Some(buf) => Ok(buf.all_notes.clone()),
        None => Err("No session loaded".to_string()),
    }
}

#[tauri::command]
pub fn clear_session() -> Result<(), String> {
    *CURRENT_BUFFER.lock().unwrap() = None;
    println!("[MIDI] Session cleared");
    Ok(())
}
