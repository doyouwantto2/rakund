use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Core file item structures for maintainability
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstrumentItem {
    pub name: String,
    pub folder: String,
    pub path: PathBuf,
    pub json_file: PathBuf,
    pub samples_dir: PathBuf,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongItem {
    pub name: String,
    pub path: PathBuf,
    pub file_type: SongFileType,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SongFileType {
    Midi,
    Other(String),
}

// Common file metadata for all items
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileMetadata {
    pub name: String,
    pub path: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub size: Option<u64>,
    pub file_type: FileType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileType {
    Instrument,
    Song,
    Directory,
    File,
}

// File operations interface
pub trait FileManager {
    type Item;
    type Error;

    // Read operations
    async fn list_files(&self) -> Result<Vec<FileMetadata>, Self::Error>;
    async fn get_file(&self, path: &str) -> Result<Self::Item, Self::Error>;
    async fn file_exists(&self, path: &str) -> Result<bool, Self::Error>;

    // Write operations
    async fn create_file(&self, item: &Self::Item) -> Result<String, Self::Error>;
    async fn update_file(&self, path: &str, item: &Self::Item) -> Result<(), Self::Error>;
    async fn delete_file(&self, path: &str) -> Result<(), Self::Error>;

    // File validation
    async fn validate_file(&self, item: &Self::Item) -> Result<(), Self::Error>;
}

// Specific traits for different file types
pub trait InstrumentFileManager: FileManager<Item = InstrumentItem, Error = StorageError> {
    async fn scan_instrument_directories(&self) -> Result<Vec<PathBuf>, StorageError>;
    async fn get_instrument_directory(&self, folder: &str) -> Result<InstrumentItem, StorageError>;
    async fn validate_instrument_structure(&self, path: &PathBuf) -> Result<(), StorageError>;
}

pub trait SongFileManager: FileManager<Item = SongItem, Error = StorageError> {
    async fn scan_song_files(&self) -> Result<Vec<SongFile>, StorageError>;
    async fn get_song_file(&self, path: &str) -> Result<SongItem, StorageError>;
    async fn validate_song_file(&self, path: &PathBuf) -> Result<(), StorageError>;
}

// File operation results
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongFile {
    pub name: String,
    pub path: String,
    pub file_type: SongFileType,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageError {
    pub message: String,
    pub error_type: StorageErrorType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StorageErrorType {
    NotFound,
    InvalidFile,
    IoError,
    ParseError,
    PermissionDenied,
    InvalidStructure,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}

impl std::fmt::Display for StorageErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageErrorType::NotFound => write!(f, "NotFound"),
            StorageErrorType::InvalidFile => write!(f, "InvalidFile"),
            StorageErrorType::IoError => write!(f, "IoError"),
            StorageErrorType::ParseError => write!(f, "ParseError"),
            StorageErrorType::PermissionDenied => write!(f, "PermissionDenied"),
            StorageErrorType::InvalidStructure => write!(f, "InvalidStructure"),
        }
    }
}

impl std::error::Error for StorageError {}

// Response types for Tauri commands (file info only)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstrumentFileResponse {
    pub name: String,
    pub folder: String,
    pub path: String,
    pub json_file: String,
    pub samples_dir: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongFileResponse {
    pub name: String,
    pub path: String,
    pub file_type: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub size: Option<u64>,
}

impl From<&InstrumentItem> for InstrumentFileResponse {
    fn from(item: &InstrumentItem) -> Self {
        Self {
            name: item.name.clone(),
            folder: item.folder.clone(),
            path: item.path.to_string_lossy().to_string(),
            json_file: item.json_file.to_string_lossy().to_string(),
            samples_dir: item.samples_dir.to_string_lossy().to_string(),
            created_at: item.created_at.clone(),
            modified_at: item.modified_at.clone(),
            size: item.size,
        }
    }
}

impl InstrumentFileResponse {
    pub fn from_config(config: &crate::setup::config::InstrumentConfig, folder: &str) -> Self {
        Self {
            name: config.instrument.clone(),
            folder: folder.to_string(),
            path: crate::storage::basic::BasicFileOperations::get_instrument_path(folder)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            json_file: crate::storage::basic::BasicFileOperations::get_instrument_json_path(folder)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            samples_dir: crate::storage::basic::BasicFileOperations::get_instrument_samples_path(folder)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            created_at: None,
            modified_at: None,
            size: None,
        }
    }
}

impl From<&SongItem> for SongFileResponse {
    fn from(item: &SongItem) -> Self {
        Self {
            name: item.name.clone(),
            path: item.path.to_string_lossy().to_string(),
            file_type: match &item.file_type {
                SongFileType::Midi => "midi".to_string(),
                SongFileType::Other(other) => other.clone(),
            },
            created_at: item.created_at.clone(),
            modified_at: item.modified_at.clone(),
            size: item.size,
        }
    }
}
