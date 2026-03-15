use crate::storage::basic::BasicFileOperations;
use crate::storage::*;
use std::path::{Path, PathBuf};

// Instrument file operations - pure functions only
pub fn new_instrument_file_manager() -> Result<PathBuf, StorageError> {
    BasicFileOperations::get_instruments_dir()
}

// Get all instrument directories that have instrument.json
pub fn get_valid_instrument_directories() -> Result<Vec<PathBuf>, StorageError> {
    let instruments_dir = BasicFileOperations::get_instruments_dir()?;
    let directories = BasicFileOperations::list_directories(&instruments_dir)?;
    Ok(directories
        .into_iter()
        .filter(|e| e.join("instrument.json").exists())
        .collect())
}

// Get instrument directory by folder name
pub fn get_instrument_directory(folder: &str) -> Result<PathBuf, StorageError> {
    let instrument_dir = new_instrument_file_manager()?.join(folder);
    BasicFileOperations::validate_instrument_structure(&instrument_dir)?;
    Ok(instrument_dir)
}

// Validate instrument structure
pub fn validate_instrument_structure(path: &PathBuf) -> Result<(), StorageError> {
    BasicFileOperations::validate_instrument_structure(path)
}

// Read instrument.json file content
pub fn read_instrument_config(folder: &str) -> Result<String, StorageError> {
    let instrument_dir = get_instrument_directory(folder)?;
    let json_path = instrument_dir.join("instrument.json");
    BasicFileOperations::read_file_content(&json_path)
}

// Get all sample paths for an instrument
pub fn get_sample_paths(folder: &str) -> Result<Vec<PathBuf>, StorageError> {
    let instrument_dir = get_instrument_directory(folder)?;
    let samples_dir = instrument_dir.join("samples");
    
    if BasicFileOperations::directory_exists(&samples_dir) {
        BasicFileOperations::list_files(&samples_dir)
    } else {
        Ok(Vec::new())
    }
}

// Song file operations - pure functions only
pub fn new_song_file_manager() -> Result<PathBuf, StorageError> {
    BasicFileOperations::get_songs_dir()
}

// Get all song files
pub fn get_all_song_files() -> Result<Vec<PathBuf>, StorageError> {
    let songs_dir = new_song_file_manager()?;
    BasicFileOperations::list_files(&songs_dir)
}

// Get song file by path
pub fn get_song_file_info(path: &Path) -> Result<Option<SongFile>, StorageError> {
    let metadata = BasicFileOperations::get_file_metadata(path)?;
    
    let file_type = BasicFileOperations::get_song_file_type(path);

    Ok(Some(SongFile {
        name: metadata.name,
        path: metadata.path,
        file_type,
        created_at: metadata.created_at,
        modified_at: metadata.modified_at,
        size: metadata.size,
    }))
}

// Validate song file
pub fn validate_song_file(path: &PathBuf) -> Result<(), StorageError> {
    BasicFileOperations::validate_song_file(path)
}

// Read song file content
pub fn read_song_file(path: &str) -> Result<Vec<u8>, StorageError> {
    let songs_dir = new_song_file_manager()?;
    let song_path = songs_dir.join(path);
    BasicFileOperations::read_file_bytes(&song_path)
}

// Simple sync file operations
pub fn get_instrument_file_sync(path: &str) -> Result<InstrumentItem, StorageError> {
    let instrument_path = PathBuf::from(path);
    BasicFileOperations::validate_instrument_structure(&instrument_path)?;
    Ok(BasicFileOperations::create_instrument_item(&instrument_path)?)
}

pub fn instrument_file_exists_sync(path: &str) -> Result<bool, StorageError> {
    let path = PathBuf::from(path);
    Ok(BasicFileOperations::directory_exists(&path))
}

pub fn list_instrument_files_sync() -> Result<Vec<FileMetadata>, StorageError> {
    let directories = get_valid_instrument_directories()?;
    let mut files = Vec::new();
    
    for dir in directories {
        let metadata = BasicFileOperations::get_file_metadata(&dir)?;
        files.push(FileMetadata {
            file_type: FileType::Instrument,
            ..metadata
        });
    }
    
    Ok(files)
}

pub fn create_instrument_file_sync(item: &InstrumentItem) -> Result<String, StorageError> {
    BasicFileOperations::create_directory(&item.path)?;
    BasicFileOperations::create_directory(&item.samples_dir)?;
    Ok(item.path.to_string_lossy().to_string())
}

pub fn delete_instrument_file_sync(path: &str) -> Result<(), StorageError> {
    let path = PathBuf::from(path);
    BasicFileOperations::delete_directory(&path)
}

pub fn scan_instrument_directories_sync() -> Result<Vec<PathBuf>, StorageError> {
    let directories = get_valid_instrument_directories()?;
    let mut valid_directories = Vec::new();
    for dir in directories {
        if BasicFileOperations::validate_instrument_structure(&dir).is_ok() {
            valid_directories.push(dir.clone());
        }
    }

    Ok(valid_directories)
}

pub fn get_instrument_item_sync(folder: &str) -> Result<InstrumentItem, StorageError> {
    let instrument_dir = get_instrument_directory(folder)?;
    BasicFileOperations::validate_instrument_structure(&instrument_dir)?;
    Ok(BasicFileOperations::create_instrument_item(&instrument_dir)?)
}

// Song file sync operations
pub fn get_song_file_sync(path: &str) -> Result<SongItem, StorageError> {
    let songs_dir = new_song_file_manager()?;
    let song_path = songs_dir.join(path);
    BasicFileOperations::validate_song_file(&song_path)?;
    
    let metadata = BasicFileOperations::get_file_metadata(&song_path)?;
    let file_type = BasicFileOperations::get_song_file_type(&song_path);
    
    Ok(SongItem {
        name: metadata.name,
        path: song_path,
        file_type,
        created_at: metadata.created_at,
        modified_at: metadata.modified_at,
        size: metadata.size,
    })
}

pub fn song_file_exists_sync(path: &str) -> Result<bool, StorageError> {
    let songs_dir = new_song_file_manager()?;
    let path = songs_dir.join(path);
    Ok(BasicFileOperations::file_exists(&path))
}

pub fn list_song_files_sync() -> Result<Vec<FileMetadata>, StorageError> {
    let files = get_all_song_files()?;
    let mut metadata_list = Vec::new();

    for file in files {
        let metadata = BasicFileOperations::get_file_metadata(&file)?;
        metadata_list.push(FileMetadata {
            file_type: FileType::Song,
            ..metadata
        });
    }

    Ok(metadata_list)
}

pub fn create_song_file_sync(item: &SongItem) -> Result<String, StorageError> {
    BasicFileOperations::create_file(&item.path, "")?;
    Ok(item.path.to_string_lossy().to_string())
}

pub fn delete_song_file_sync(path: &str) -> Result<(), StorageError> {
    let songs_dir = new_song_file_manager()?;
    let path = songs_dir.join(path);
    BasicFileOperations::delete_file(&path)
}

pub fn scan_song_files_sync() -> Result<Vec<SongFile>, StorageError> {
    let files = get_all_song_files()?;

    let mut song_files = Vec::new();
    for file in files {
        if let Some(song_file) = BasicFileOperations::create_song_item(&file) {
            song_files.push(song_file);
        }
    }

    Ok(song_files)
}
