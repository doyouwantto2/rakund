use crate::storage::items::{FileManager as FileManagerTrait, *};
use crate::storage::logic::{InstrumentFileManagerImpl, SongFileManagerImpl};
use std::sync::Arc;
use tokio::sync::RwLock;

// File handler responsible for folder and file fetching operations
// Used by player.rs, visualizer.rs, and manager.rs
pub struct FileHandler {
    pub instrument_manager: Arc<RwLock<InstrumentFileManagerImpl>>,
    pub song_manager: Arc<RwLock<SongFileManagerImpl>>,
}

impl FileHandler {
    pub fn new() -> Result<Self, StorageError> {
        let instrument_manager = Arc::new(RwLock::new(InstrumentFileManagerImpl::new()?));
        let song_manager = Arc::new(RwLock::new(SongFileManagerImpl::new()?));

        Ok(Self {
            instrument_manager,
            song_manager,
        })
    }

    // Instrument operations for player.rs and visualizer.rs
    pub async fn scan_instrument_directories(
        &self,
    ) -> Result<Vec<std::path::PathBuf>, StorageError> {
        let manager = self.instrument_manager.read().await;
        manager.scan_instrument_directories().await
    }

    pub async fn get_instrument_directory(
        &self,
        folder: &str,
    ) -> Result<InstrumentItem, StorageError> {
        let manager = self.instrument_manager.read().await;
        manager.get_instrument_directory(folder).await
    }

    pub async fn instrument_exists(&self, folder: &str) -> Result<bool, StorageError> {
        let instrument_path =
            crate::storage::basic::BasicFileOperations::get_instrument_path(folder)?;
        Ok(crate::storage::basic::BasicFileOperations::directory_exists(&instrument_path))
    }

    // Song operations for visualizer.rs
    pub async fn scan_song_files(&self) -> Result<Vec<SongFile>, StorageError> {
        let manager = self.song_manager.read().await;
        manager.scan_song_files().await
    }

    pub async fn get_song_file(&self, path: &str) -> Result<SongItem, StorageError> {
        let manager = self.song_manager.read().await;
        manager.get_song_file(path).await
    }

    pub async fn song_exists(&self, path: &str) -> Result<bool, StorageError> {
        let manager = self.song_manager.read().await;
        manager.file_exists(path).await
    }

    // File metadata operations
    pub async fn get_instrument_metadata(
        &self,
        folder: &str,
    ) -> Result<FileMetadata, StorageError> {
        let manager = self.instrument_manager.read().await;
        let path = crate::storage::basic::BasicFileOperations::get_instrument_path(folder)?;
        let files = manager.list_files().await?;
        files
            .into_iter()
            .find(|f| f.path == path.to_string_lossy())
            .ok_or(StorageError {
                message: "Instrument not found".to_string(),
                error_type: StorageErrorType::NotFound,
            })
    }

    pub async fn get_song_metadata(&self, path: &str) -> Result<FileMetadata, StorageError> {
        let manager = self.song_manager.read().await;
        let files = manager.list_files().await?;
        files
            .into_iter()
            .find(|f| f.path == path)
            .ok_or(StorageError {
                message: "Song not found".to_string(),
                error_type: StorageErrorType::NotFound,
            })
    }

    // Validation operations
    pub async fn validate_instrument_structure(&self, folder: &str) -> Result<(), StorageError> {
        let manager = self.instrument_manager.read().await;
        let path = crate::storage::basic::BasicFileOperations::get_instrument_path(folder)?;
        manager.validate_instrument_structure(&path).await
    }

    pub async fn validate_song_file(&self, path: &str) -> Result<(), StorageError> {
        let manager = self.song_manager.read().await;
        let path_buf = std::path::PathBuf::from(path);
        manager.validate_song_file(&path_buf).await
    }

    // CRUD operations for manager.rs
    pub async fn list_instruments(&self) -> Result<Vec<InstrumentFileResponse>, StorageError> {
        let manager = self.instrument_manager.read().await;
        let files = manager.list_files().await?;

        let mut responses = Vec::new();
        for file_metadata in files {
            if let FileType::Instrument = file_metadata.file_type {
                let instrument_item = manager.get_file(&file_metadata.path).await?;
                responses.push(InstrumentFileResponse::from(&instrument_item));
            }
        }

        Ok(responses)
    }

    pub async fn list_songs(&self) -> Result<Vec<SongFileResponse>, StorageError> {
        let manager = self.song_manager.read().await;
        let files = manager.scan_song_files().await?;

        let mut responses = Vec::new();
        for song_file in files {
            responses.push(SongFileResponse {
                name: song_file.name,
                path: song_file.path,
                file_type: match song_file.file_type {
                    SongFileType::Midi => "midi".to_string(),
                    SongFileType::Other(other) => other,
                },
                created_at: song_file.created_at,
                modified_at: song_file.modified_at,
                size: song_file.size,
            });
        }

        Ok(responses)
    }

    pub async fn create_instrument(
        &self,
        folder: &str,
    ) -> Result<InstrumentFileResponse, StorageError> {
        let manager = self.instrument_manager.read().await;
        let instruments_dir = &manager.instruments_dir;

        let instrument_path = instruments_dir.join(folder);
        let json_file = instrument_path.join("instrument.json");
        let samples_dir = instrument_path.join("samples");

        let instrument_item = InstrumentItem {
            name: folder.to_string(),
            folder: folder.to_string(),
            path: instrument_path,
            json_file,
            samples_dir,
            created_at: None,
            modified_at: None,
            size: None,
        };

        manager.create_file(&instrument_item).await?;
        Ok(InstrumentFileResponse::from(&instrument_item))
    }

    pub async fn delete_instrument(&self, folder: &str) -> Result<(), StorageError> {
        let manager = self.instrument_manager.read().await;
        let instrument_path = manager.instruments_dir.join(folder);
        manager
            .delete_file(&instrument_path.to_string_lossy())
            .await
    }

    pub async fn create_song(
        &self,
        name: &str,
        file_type: SongFileType,
    ) -> Result<SongFileResponse, StorageError> {
        let manager = self.song_manager.read().await;
        let songs_dir = &manager.songs_dir;

        let extension = match &file_type {
            SongFileType::Midi => "mid",
            SongFileType::Other(ext) => ext,
        };

        let song_path = songs_dir.join(format!("{}.{}", name, extension));

        let song_item = SongItem {
            name: name.to_string(),
            path: song_path.clone(),
            file_type,
            created_at: None,
            modified_at: None,
            size: None,
        };

        manager.create_file(&song_item).await?;
        Ok(SongFileResponse::from(&song_item))
    }

    pub async fn delete_song(&self, path: &str) -> Result<(), StorageError> {
        let manager = self.song_manager.read().await;
        manager.delete_file(path).await
    }

    pub async fn get_file_metadata(
        &self,
        path: &str,
        file_type: FileType,
    ) -> Result<FileMetadata, StorageError> {
        match file_type {
            FileType::Instrument => {
                let manager = self.instrument_manager.read().await;
                let files: Vec<FileMetadata> = manager.list_files().await?;
                files
                    .into_iter()
                    .find(|f| f.path == path)
                    .ok_or(StorageError {
                        message: "File not found".to_string(),
                        error_type: StorageErrorType::NotFound,
                    })
            }
            FileType::Song => {
                let manager = self.song_manager.read().await;
                let files: Vec<FileMetadata> = manager.list_files().await?;
                files
                    .into_iter()
                    .find(|f| f.path == path)
                    .ok_or(StorageError {
                        message: "File not found".to_string(),
                        error_type: StorageErrorType::NotFound,
                    })
            }
            _ => Err(StorageError {
                message: "Unsupported file type".to_string(),
                error_type: StorageErrorType::InvalidFile,
            }),
        }
    }
}

// Tauri state management
impl Default for FileHandler {
    fn default() -> Self {
        Self::new().expect("Failed to initialize FileHandler")
    }
}

// Type alias for backward compatibility
pub type FileManager = FileHandler;

