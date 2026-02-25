use crate::storage::items::*;
use crate::storage::basic::BasicFileOperations;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

// Instrument file manager implementation - specific logic only
pub struct InstrumentFileManagerImpl {
    pub instruments_dir: PathBuf,
}

impl InstrumentFileManagerImpl {
    pub fn new() -> Result<Self, StorageError> {
        Ok(Self {
            instruments_dir: BasicFileOperations::get_instruments_dir()?,
        })
    }
}

impl FileManager for InstrumentFileManagerImpl {
    type Item = InstrumentItem;
    type Error = StorageError;

    async fn list_files(&self) -> Result<Vec<FileMetadata>, Self::Error> {
        let directories = BasicFileOperations::list_directories(&self.instruments_dir)?;
        
        let mut files = Vec::new();
        for dir in directories {
            if let Ok(metadata) = BasicFileOperations::get_file_metadata(&dir) {
                files.push(metadata);
            }
        }
        
        Ok(files)
    }

    async fn get_file(&self, path: &str) -> Result<Self::Item, Self::Error> {
        let instrument_path = PathBuf::from(path);
        self.validate_instrument_structure(&instrument_path).await?;

        let json_file = instrument_path.join("instrument.json");
        let samples_dir = instrument_path.join("samples");

        let metadata = BasicFileOperations::get_file_metadata(&instrument_path)?;

        let folder_name = instrument_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(InstrumentItem {
            name: folder_name.clone(),
            folder: folder_name,
            path: instrument_path,
            json_file,
            samples_dir,
            created_at: metadata.created_at,
            modified_at: metadata.modified_at,
            size: metadata.size,
        })
    }

    async fn file_exists(&self, path: &str) -> Result<bool, Self::Error> {
        let path = PathBuf::from(path);
        Ok(BasicFileOperations::directory_exists(&path))
    }

    async fn create_file(&self, item: &Self::Item) -> Result<String, Self::Error> {
        BasicFileOperations::create_directory(&item.path)?;
        BasicFileOperations::create_directory(&item.samples_dir)?;
        
        // Create empty instrument.json if it doesn't exist
        if !BasicFileOperations::file_exists(&item.json_file) {
            BasicFileOperations::create_file(&item.json_file, "{}")?;
        }

        Ok(item.path.to_string_lossy().to_string())
    }

    async fn update_file(&self, path: &str, _item: &Self::Item) -> Result<(), Self::Error> {
        // For instruments, updating typically means updating the JSON file
        // This would be handled by the data layer, not the file layer
        self.validate_instrument_structure(&PathBuf::from(path)).await
    }

    async fn delete_file(&self, path: &str) -> Result<(), Self::Error> {
        let path = PathBuf::from(path);
        BasicFileOperations::delete_directory(&path)
    }

    async fn validate_file(&self, item: &Self::Item) -> Result<(), Self::Error> {
        self.validate_instrument_structure(&item.path).await
    }
}

impl InstrumentFileManager for InstrumentFileManagerImpl {
    async fn scan_instrument_directories(&self) -> Result<Vec<PathBuf>, StorageError> {
        let directories = BasicFileOperations::list_directories(&self.instruments_dir)?;
        
        let mut valid_directories = Vec::new();
        for dir in directories {
            if self.validate_instrument_structure(&dir).await.is_ok() {
                valid_directories.push(dir);
            }
        }
        
        Ok(valid_directories)
    }

    async fn get_instrument_directory(&self, folder: &str) -> Result<InstrumentItem, StorageError> {
        let instrument_path = BasicFileOperations::get_instrument_path(folder)?;
        self.get_file(&instrument_path.to_string_lossy()).await
    }

    async fn validate_instrument_structure(&self, path: &PathBuf) -> Result<(), StorageError> {
        if !BasicFileOperations::directory_exists(path) {
            return Err(StorageError {
                message: "Instrument directory does not exist".to_string(),
                error_type: StorageErrorType::NotFound,
            });
        }

        let json_file = path.join("instrument.json");
        if !BasicFileOperations::file_exists(&json_file) {
            return Err(StorageError {
                message: "instrument.json not found".to_string(),
                error_type: StorageErrorType::InvalidStructure,
            });
        }

        // That's it! The instrument.json contains all the sample paths,
        // so we don't need to validate directory structure
        Ok(())
    }
}

// Song file manager implementation - specific logic only
pub struct SongFileManagerImpl {
    pub songs_dir: PathBuf,
}

impl SongFileManagerImpl {
    pub fn new() -> Result<Self, StorageError> {
        Ok(Self {
            songs_dir: BasicFileOperations::get_songs_dir()?,
        })
    }
}

impl FileManager for SongFileManagerImpl {
    type Item = SongItem;
    type Error = StorageError;

    async fn list_files(&self) -> Result<Vec<FileMetadata>, Self::Error> {
        let files = BasicFileOperations::list_files(&self.songs_dir)?;
        
        let mut metadata_list = Vec::new();
        for file in files {
            if let Ok(metadata) = BasicFileOperations::get_file_metadata(&file) {
                metadata_list.push(metadata);
            }
        }
        
        Ok(metadata_list)
    }

    async fn get_file(&self, path: &str) -> Result<Self::Item, Self::Error> {
        let song_path = PathBuf::from(path);
        self.validate_song_file(&song_path).await?;

        let metadata = BasicFileOperations::get_file_metadata(&song_path)?;

        let file_name = song_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let file_type = BasicFileOperations::get_song_file_type(&song_path);

        Ok(SongItem {
            name: file_name.clone(),
            path: song_path,
            file_type,
            created_at: metadata.created_at,
            modified_at: metadata.modified_at,
            size: metadata.size,
        })
    }

    async fn file_exists(&self, path: &str) -> Result<bool, Self::Error> {
        let path = PathBuf::from(path);
        Ok(BasicFileOperations::file_exists(&path))
    }

    async fn create_file(&self, item: &Self::Item) -> Result<String, Self::Error> {
        BasicFileOperations::create_file(&item.path, "")?;
        Ok(item.path.to_string_lossy().to_string())
    }

    async fn update_file(&self, path: &str, _item: &Self::Item) -> Result<(), Self::Error> {
        // For songs, updating would be handled by the data layer
        self.validate_song_file(&PathBuf::from(path)).await
    }

    async fn delete_file(&self, path: &str) -> Result<(), Self::Error> {
        let path = PathBuf::from(path);
        BasicFileOperations::delete_file(&path)
    }

    async fn validate_file(&self, item: &Self::Item) -> Result<(), Self::Error> {
        self.validate_song_file(&item.path).await
    }
}

impl SongFileManager for SongFileManagerImpl {
    async fn scan_song_files(&self) -> Result<Vec<SongFile>, StorageError> {
        let files = BasicFileOperations::list_files(&self.songs_dir)?;
        
        let mut song_files = Vec::new();
        for file in files {
            if let Some(metadata) = self.get_song_file_info(&file) {
                song_files.push(metadata);
            }
        }
        
        Ok(song_files)
    }

    async fn get_song_file(&self, path: &str) -> Result<SongItem, StorageError> {
        self.get_file(path).await
    }

    async fn validate_song_file(&self, path: &PathBuf) -> Result<(), StorageError> {
        if !BasicFileOperations::file_exists(path) {
            return Err(StorageError {
                message: "Song file does not exist".to_string(),
                error_type: StorageErrorType::NotFound,
            });
        }

        Ok(())
    }
}

impl SongFileManagerImpl {
    fn get_song_file_info(&self, path: &Path) -> Option<SongFile> {
        let metadata = BasicFileOperations::get_file_metadata(path).ok()?;
        
        let file_type = BasicFileOperations::get_song_file_type(path);

        Some(SongFile {
            name: metadata.name,
            path: metadata.path,
            file_type,
            created_at: metadata.created_at,
            modified_at: metadata.modified_at,
            size: metadata.size,
        })
    }
}