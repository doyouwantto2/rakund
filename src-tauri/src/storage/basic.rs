use crate::storage::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct BasicFileOperations;

impl BasicFileOperations {
    pub fn get_instruments_dir() -> Result<PathBuf, StorageError> {
        let base = dirs_next::config_dir()
            .ok_or_else(|| StorageError {
                message: "Cannot find config directory".to_string(),
                error_type: StorageErrorType::IoError,
            })?;
        let dir = base.join("rakund").join("instruments");
        fs::create_dir_all(&dir).map_err(|e| StorageError {
            message: format!("Cannot create instruments dir: {}", e),
            error_type: StorageErrorType::IoError,
        })?;
        Ok(dir)
    }

    pub fn get_songs_dir() -> Result<PathBuf, StorageError> {
        let base = dirs_next::config_dir()
            .ok_or_else(|| StorageError {
                message: "Cannot find config directory".to_string(),
                error_type: StorageErrorType::IoError,
            })?;
        let dir = base.join("rakund").join("songs");
        fs::create_dir_all(&dir).map_err(|e| StorageError {
            message: format!("Cannot create songs dir: {}", e),
            error_type: StorageErrorType::IoError,
        })?;
        Ok(dir)
    }

    pub fn directory_exists(path: &Path) -> bool {
        path.exists() && path.is_dir()
    }

    pub fn file_exists(path: &Path) -> bool {
        path.exists() && path.is_file()
    }

    pub fn create_directory(path: &Path) -> Result<(), StorageError> {
        fs::create_dir_all(path).map_err(|e| StorageError {
            message: format!("Failed to create directory {:?}: {}", path, e),
            error_type: StorageErrorType::IoError,
        })
    }

    pub fn create_file(path: &Path, content: &str) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                Self::create_directory(parent)?;
            }
        }

        fs::write(path, content).map_err(|e| StorageError {
            message: format!("Failed to create file {:?}: {}", path, e),
            error_type: StorageErrorType::IoError,
        })
    }

    pub fn delete_directory(path: &Path) -> Result<(), StorageError> {
        fs::remove_dir_all(path).map_err(|e| StorageError {
            message: format!("Failed to delete directory {:?}: {}", path, e),
            error_type: StorageErrorType::IoError,
        })
    }

    pub fn delete_file(path: &Path) -> Result<(), StorageError> {
        fs::remove_file(path).map_err(|e| StorageError {
            message: format!("Failed to delete file {:?}: {}", path, e),
            error_type: StorageErrorType::IoError,
        })
    }

    // File metadata operations
    pub fn get_file_metadata(path: &Path) -> Result<FileMetadata, StorageError> {
        let metadata = fs::metadata(path).map_err(|e| StorageError {
            message: format!("Failed to get metadata for {:?}: {}", path, e),
            error_type: StorageErrorType::IoError,
        })?;

        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let path_str = path.to_string_lossy().to_string();

        let created_at = metadata
            .created()
            .ok()
            .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs().to_string());

        let modified_at = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs().to_string());

        let size = Some(metadata.len());

        let file_type = if path.is_dir() {
            FileType::Directory
        } else {
            FileType::File
        };

        Ok(FileMetadata {
            name,
            path: path_str,
            created_at,
            modified_at,
            size,
            file_type,
        })
    }

    // Directory listing operations
    pub fn list_directories(base_dir: &Path) -> Result<Vec<PathBuf>, StorageError> {
        if !base_dir.exists() {
            return Ok(vec![]);
        }

        let entries = fs::read_dir(base_dir).map_err(|e| StorageError {
            message: format!("Failed to read directory {:?}: {}", base_dir, e),
            error_type: StorageErrorType::IoError,
        })?;

        let mut directories = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| StorageError {
                message: format!("Failed to read directory entry: {}", e),
                error_type: StorageErrorType::IoError,
            })?;

            let path = entry.path();
            if path.is_dir() {
                directories.push(path);
            }
        }

        Ok(directories)
    }

    pub fn list_files(base_dir: &Path) -> Result<Vec<PathBuf>, StorageError> {
        if !base_dir.exists() {
            return Ok(vec![]);
        }

        let entries = fs::read_dir(base_dir).map_err(|e| StorageError {
            message: format!("Failed to read directory {:?}: {}", base_dir, e),
            error_type: StorageErrorType::IoError,
        })?;

        let mut files = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| StorageError {
                message: format!("Failed to read directory entry: {}", e),
                error_type: StorageErrorType::IoError,
            })?;

            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }

        Ok(files)
    }

    // File content operations
    pub fn read_file_content(path: &Path) -> Result<String, StorageError> {
        std::fs::read_to_string(path).map_err(|e| StorageError {
            message: format!("Failed to read file {}: {}", path.display(), e),
            error_type: StorageErrorType::IoError,
        })
    }

    pub fn read_file_bytes(path: &Path) -> Result<Vec<u8>, StorageError> {
        std::fs::read(path).map_err(|e| StorageError {
            message: format!("Failed to read file {}: {}", path.display(), e),
            error_type: StorageErrorType::IoError,
        })
    }

    pub fn write_file_content(path: &Path, content: &str) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                Self::create_directory(parent)?;
            }
        }

        fs::write(path, content).map_err(|e| StorageError {
            message: format!("Failed to write file {:?}: {}", path, e),
            error_type: StorageErrorType::IoError,
        })
    }

    // Path utilities
    pub fn get_instrument_path(folder: &str) -> Result<PathBuf, StorageError> {
        let instruments_dir = Self::get_instruments_dir()?;
        Ok(instruments_dir.join(folder))
    }

    pub fn get_instrument_json_path(folder: &str) -> Result<PathBuf, StorageError> {
        let instrument_path = Self::get_instrument_path(folder)?;
        Ok(instrument_path.join("instrument.json"))
    }

    pub fn get_instrument_samples_path(folder: &str) -> Result<PathBuf, StorageError> {
        let instrument_path = Self::get_instrument_path(folder)?;
        Ok(instrument_path.join("samples"))
    }

    pub fn get_song_path(name: &str, extension: &str) -> Result<PathBuf, StorageError> {
        let songs_dir = Self::get_songs_dir()?;
        Ok(songs_dir.join(format!("{}.{}", name, extension)))
    }

    // File type detection
    pub fn is_midi_file(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("mid") || ext.eq_ignore_ascii_case("midi"))
            .unwrap_or(false)
    }

    pub fn get_song_file_type(path: &Path) -> SongFileType {
        if Self::is_midi_file(path) {
            SongFileType::Midi
        } else {
            SongFileType::Other(
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
            )
        }
    }

    // Instrument validation operations
    pub fn validate_instrument_structure(path: &Path) -> Result<(), StorageError> {
        if !Self::directory_exists(path) {
            return Err(StorageError {
                message: "Instrument directory does not exist".to_string(),
                error_type: StorageErrorType::NotFound,
            });
        }

        let json_file = path.join("instrument.json");
        if !Self::file_exists(&json_file) {
            return Err(StorageError {
                message: "instrument.json not found".to_string(),
                error_type: StorageErrorType::InvalidStructure,
            });
        }

        // That's it! The instrument.json contains all the sample paths,
        // so we don't need to validate directory structure
        Ok(())
    }

    // Song validation operations
    pub fn validate_song_file(path: &Path) -> Result<(), StorageError> {
        if !Self::file_exists(path) {
            return Err(StorageError {
                message: "Song file does not exist".to_string(),
                error_type: StorageErrorType::NotFound,
            });
        }

        Ok(())
    }

    // Instrument item creation
    pub fn create_instrument_item(path: &Path) -> Result<InstrumentItem, StorageError> {
        let metadata = Self::get_file_metadata(path)?;

        let folder_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let json_file = path.join("instrument.json");
        let samples_dir = path.join("samples");

        Ok(InstrumentItem {
            name: folder_name.clone(),
            folder: folder_name,
            path: path.to_path_buf(),
            json_file,
            samples_dir,
            created_at: metadata.created_at,
            modified_at: metadata.modified_at,
            size: metadata.size,
        })
    }

    // Song item creation
    pub fn create_song_item(path: &Path) -> Option<SongFile> {
        let metadata = Self::get_file_metadata(path).ok()?;

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let file_type = Self::get_song_file_type(path);

        Some(SongFile {
            name: file_name.clone(),
            path: path.to_string_lossy().to_string(),
            file_type,
            created_at: metadata.created_at,
            modified_at: metadata.modified_at,
            size: metadata.size,
        })
    }
}
