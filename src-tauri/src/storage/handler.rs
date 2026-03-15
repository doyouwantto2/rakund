use crate::storage::basic::BasicFileOperations;
use crate::storage::*;
use crate::storage::logic::*;
use std::path::PathBuf;

// File handler responsible for folder and file fetching operations
// Used by player.rs, visualizer.rs, and manager.rs
pub struct FileHandler {
    // No internal state - just orchestrates the pure functions
}

impl FileHandler {
    pub fn new() -> Result<Self, StorageError> {
        Ok(Self {})
    }

    // Instrument file operations
    pub fn instrument_exists(&self, folder: &str) -> Result<bool, StorageError> {
        let instrument_path = BasicFileOperations::get_instrument_path(folder)?;
        instrument_file_exists_sync(&instrument_path.to_string_lossy())
    }

    pub fn scan_instrument_directories(&self) -> Result<Vec<PathBuf>, StorageError> {
        scan_instrument_directories_sync()
    }

    pub fn get_instrument_directory(&self, folder: &str) -> Result<InstrumentItem, StorageError> {
        get_instrument_item_sync(folder)
    }

    // Song operations for visualizer.rs
    pub fn scan_song_files(&self) -> Result<Vec<SongFile>, StorageError> {
        scan_song_files_sync()
    }

    pub fn get_song_file(&self, path: &str) -> Result<SongItem, StorageError> {
        get_song_file_sync(path)
    }

    pub fn song_exists(&self, path: &str) -> Result<bool, StorageError> {
        song_file_exists_sync(path)
    }

    pub fn validate_instrument_structure(&self, folder: &str) -> Result<(), StorageError> {
        let instrument_path = BasicFileOperations::get_instrument_path(folder)?;
        validate_instrument_structure(&instrument_path)
    }

    pub fn validate_song_file(&self, path_buf: &PathBuf) -> Result<(), StorageError> {
        validate_song_file(path_buf)
    }

    // Additional methods needed by manager.rs
    pub fn create_instrument_file_sync(&self, item: &InstrumentItem) -> Result<String, StorageError> {
        create_instrument_file_sync(item)
    }

    pub fn delete_instrument_file_sync(&self, path: &str) -> Result<(), StorageError> {
        delete_instrument_file_sync(path)
    }

    pub fn create_song_file_sync(&self, item: &SongItem) -> Result<String, StorageError> {
        create_song_file_sync(item)
    }

    pub fn delete_song_file_sync(&self, path: &str) -> Result<(), StorageError> {
        delete_song_file_sync(path)
    }
}

// Re-export for convenience
pub type FileManager = FileHandler;
