use crate::extra::challenge::buffer::{MidiBuffer, MidiNoteMs};
use crate::extra::challenge::engine::decoder::MidiParser;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VisualizerError {
    #[error("Failed to load MIDI file: {0}")]
    LoadError(#[from] crate::extra::challenge::engine::decoder::MidiParseError),

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
pub async fn scan_songs() -> Result<Vec<SongInfo>, String> {
    use crate::storage::handler::FileHandler;
    
    let file_handler = FileHandler::new()
        .map_err(|e| e.to_string())?;
    
    let song_files = file_handler.scan_song_files().await
        .map_err(|e| e.to_string())?;

    let songs: Vec<SongInfo> = song_files
        .into_iter()
        .map(|song_file| {
            let path = std::path::PathBuf::from(&song_file.path);
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
                file_name: song_file.name,
                file_path: song_file.path,
                display_name,
            }
        })
        .collect();

    println!("[SONGS] Found {} MIDI files", songs.len());
    Ok(songs)
}

// Backward compatibility alias
#[tauri::command]
pub async fn scan_song_files() -> Result<Vec<SongInfo>, String> {
    // Just call the main function
    scan_songs().await
}

#[tauri::command]
pub async fn load_midi_session(file_path: String) -> Result<MidiSessionInfo, String> {
    use crate::storage::handler::FileHandler;
    
    // Validate song file exists using FileHandler
    let file_handler = FileHandler::new()
        .map_err(|e| e.to_string())?;
    
    let song_exists = file_handler.song_exists(&file_path).await
        .map_err(|e| e.to_string())?;
    
    if !song_exists {
        return Err(format!("Song file '{}' does not exist", file_path));
    }
    
    // Validate song file
    file_handler.validate_song_file(&file_path).await
        .map_err(|e| e.to_string())?;
    
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
    Ok(())
}
