pub mod basic;
pub mod logic;
pub mod handler;

// Re-export types from sketch directory
pub use crate::extra::sketch::instrument::response::InstrumentInfoResponse;

// Re-export error types
pub use crate::error::AudioError;

// Simple storage error type for storage operations
#[derive(Debug, Clone)]
pub struct StorageError {
    pub message: String,
    pub error_type: StorageErrorType,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.message, self.error_type)
    }
}

impl std::error::Error for StorageError {}

#[derive(Debug, Clone)]
pub enum StorageErrorType {
    IoError,
    NotFound,
    InvalidFile,
    InvalidStructure,
}

// File metadata for storage operations
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileMetadata {
    pub name: String,
    pub path: String,
    pub file_type: FileType,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub size: Option<u64>,
}

impl tauri::ipc::private::ResultFutureKind for FileMetadata {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FileType {
    File,
    Directory,
    Instrument,
    Song,
}

// Song file type enum
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SongFileType {
    Midi,
    Other(String),
}

// Basic structs for storage operations
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstrumentItem {
    pub name: String,
    pub folder: String,
    pub path: std::path::PathBuf,
    pub json_file: std::path::PathBuf,
    pub samples_dir: std::path::PathBuf,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SongItem {
    pub name: String,
    pub path: std::path::PathBuf,
    pub file_type: SongFileType,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SongFile {
    pub name: String,
    pub path: String,
    pub file_type: SongFileType,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub size: Option<u64>,
}

// Response types for Tauri commands
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstrumentFileResponse {
    pub instruments: Vec<InstrumentItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SongFileResponse {
    pub songs: Vec<SongFile>,
}

// Re-export pure functions from logic
pub use logic::{
    // Instrument operations
    new_instrument_file_manager, get_valid_instrument_directories, get_instrument_directory,
    validate_instrument_structure, read_instrument_config, get_sample_paths,
    
    // Song operations  
    new_song_file_manager, get_all_song_files, get_song_file_info, validate_song_file,
    read_song_file,
    
    // Sync file operations
    get_instrument_file_sync, instrument_file_exists_sync, list_instrument_files_sync, 
    create_instrument_file_sync, delete_instrument_file_sync, scan_instrument_directories_sync,
    get_instrument_item_sync,
    
    get_song_file_sync, song_file_exists_sync, list_song_files_sync, create_song_file_sync,
    delete_song_file_sync, scan_song_files_sync,
};

pub use basic::BasicFileOperations;
pub use handler::{FileHandler, FileManager as FileHandlerManager};
