use crate::storage::*;
use crate::storage::handler::FileHandler;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;

// Tauri commands for frontend integration only
// Manager is just for Tauri commands, no pure functions

// CRUD operations for file management
#[tauri::command]
pub async fn create_instrument(
    folder: String,
    file_handler: State<'_, Arc<RwLock<FileHandler>>>,
) -> Result<InstrumentFileResponse, String> {
    let handler = file_handler.read().await;
    let item = InstrumentItem {
        name: folder.clone(),
        folder: folder.clone(),
        path: crate::storage::basic::BasicFileOperations::get_instrument_path(&folder).unwrap(),
        json_file: crate::storage::basic::BasicFileOperations::get_instrument_json_path(&folder).unwrap(),
        samples_dir: crate::storage::basic::BasicFileOperations::get_instrument_samples_path(&folder).unwrap(),
        created_at: None,
        modified_at: None,
        size: None,
    };
    
    handler.create_instrument_file_sync(&item)
        .map_err(|e| e.to_string())?;
    
    Ok(InstrumentFileResponse {
        instruments: vec![item],
    })
}

#[tauri::command]
pub async fn delete_instrument(
    folder: String,
    file_handler: State<'_, Arc<RwLock<FileHandler>>>,
) -> Result<(), String> {
    let handler = file_handler.read().await;
    handler.delete_instrument_file_sync(&folder)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_song(
    name: String,
    file_type: String,
    file_handler: State<'_, Arc<RwLock<FileHandler>>>,
) -> Result<SongFileResponse, String> {
    let handler = file_handler.read().await;
    let song_type = if file_type == "midi" {
        SongFileType::Midi
    } else {
        SongFileType::Other(file_type.clone())
    };
    
    let item = SongItem {
        name: name.clone(),
        path: crate::storage::basic::BasicFileOperations::get_song_path(&name, &file_type).unwrap(),
        file_type: song_type,
        created_at: None,
        modified_at: None,
        size: None,
    };
    
    handler.create_song_file_sync(&item)
        .map_err(|e| e.to_string())?;
    
    Ok(SongFileResponse {
        songs: vec![SongFile {
            name: item.name,
            path: item.path.to_string_lossy().to_string(),
            file_type: item.file_type,
            created_at: item.created_at,
            modified_at: item.modified_at,
            size: item.size,
        }],
    })
}

#[tauri::command]
pub async fn delete_song(
    path: String,
    file_handler: State<'_, Arc<RwLock<FileHandler>>>,
) -> Result<(), String> {
    let handler = file_handler.read().await;
    handler.delete_song_file_sync(&path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_file_metadata(
    path: String,
    file_type: String,
    file_handler: State<'_, Arc<RwLock<FileHandler>>>,
) -> Result<FileMetadata, String> {
    let _handler = file_handler.read().await;
    
    let path_buf = std::path::PathBuf::from(&path);
    let metadata = crate::storage::basic::BasicFileOperations::get_file_metadata(&path_buf)
        .map_err(|e| e.to_string())?;
    
    let file_type_enum = if file_type == "instrument" {
        FileType::Instrument
    } else if file_type == "song" {
        FileType::Song
    } else if path_buf.is_dir() {
        FileType::Directory
    } else {
        FileType::File
    };
    
    Ok(FileMetadata {
        file_type: file_type_enum,
        ..metadata
    })
}