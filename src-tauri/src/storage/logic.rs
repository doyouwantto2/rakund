use crate::error::AudioError;
use crate::setup::config::InstrumentConfig;
use crate::storage::basic::BasicFileOperations;
use crate::storage::items::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::Emitter;

lazy_static::lazy_static! {
    pub static ref CURRENT_INSTRUMENT: Arc<Mutex<Option<InstrumentConfig>>> =
        Arc::new(Mutex::new(None));
    pub static ref CURRENT_FOLDER: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

pub struct InstrumentFileManagerImpl {
    pub instruments_dir: PathBuf,
}

impl InstrumentFileManagerImpl {
    pub fn new() -> Result<Self, StorageError> {
        Ok(Self {
            instruments_dir: BasicFileOperations::get_instruments_dir()?,
        })
    }

    pub fn set_current_folder(folder: String) {
        *CURRENT_FOLDER.lock().unwrap() = Some(folder);
    }

    pub fn load_instrument_with_progress(
        &self,
        folder: &str,
        app: &tauri::AppHandle,
    ) -> Result<InstrumentConfig, AudioError> {
        let instrument_dir = self.instruments_dir.join(folder);
        self.load_instrument_from_path(&instrument_dir, Some(app))
    }

    pub fn load_instrument(&self, folder: &str) -> Result<InstrumentConfig, AudioError> {
        let instrument_dir = self.instruments_dir.join(folder);
        self.load_instrument_from_path(&instrument_dir, None::<&tauri::AppHandle>)
    }

    fn load_instrument_from_path(
        &self,
        instrument_dir: &Path,
        app: Option<&tauri::AppHandle>,
    ) -> Result<InstrumentConfig, AudioError> {
        let json_path = instrument_dir.join("instrument.json");

        let raw = BasicFileOperations::read_file_content(&json_path).map_err(|e| {
            AudioError::InstrumentError(format!("Cannot read instrument.json: {}", e))
        })?;

        let config = InstrumentConfig::migrate_from_old(&raw)
            .map_err(|e| AudioError::InstrumentError(format!("Invalid instrument.json: {}", e)))?;

        let fast_release = config.fast_release().unwrap_or_else(|| 0.9998);
        let slow_release = config.slow_release().unwrap_or_else(|| 0.99999);

        crate::extra::sketch::instrument::release::set(fast_release, slow_release);

        crate::engine::cache::clear();

        let mut midi_keys: Vec<u8> = config
            .piano_keys
            .keys()
            .filter_map(|k| k.parse().ok())
            .collect();
        midi_keys.sort();

        let total = midi_keys
            .iter()
            .map(|m| {
                config
                    .piano_keys
                    .get(&m.to_string())
                    .map(|k| k.samples.len())
                    .unwrap_or(0)
            })
            .sum::<usize>();
        let mut done = 0usize;
        let mut last_emitted_pct = -1i32;

        let mut file_cache: HashMap<String, Arc<Vec<f32>>> = HashMap::new();

        for midi in &midi_keys {
            let key_data = &config.piano_keys[&midi.to_string()];

            for (sample_idx, sample_info) in key_data.samples.iter().enumerate() {
                let sample_path = instrument_dir.join(&sample_info.path);
                let file_key = sample_path.to_string_lossy().to_lowercase();

                let data = if let Some(cached) = file_cache.get(&file_key) {
                    cached.clone()
                } else {
                    let decoded = crate::engine::decoder::decode(&sample_path.to_string_lossy())?;
                    file_cache.insert(file_key, decoded.clone());
                    decoded
                };

                crate::engine::cache::insert_by_index(*midi, sample_idx, data);
                done += 1;

                if let Some(handle) = app {
                    let pct = ((done as f32 / total as f32) * 100.0) as i32;
                    if pct != last_emitted_pct {
                        last_emitted_pct = pct;
                        let _ = handle.emit(
                            "load_progress",
                            serde_json::json!({
                                "progress": pct as f32,
                                "loaded": done,
                                "total": total,
                                "status": "loading"
                            }),
                        );
                    }
                }
            }
        }

        if let Some(handle) = app {
            let _ = handle.emit(
                "load_progress",
                serde_json::json!({
                    "progress": 100.0,
                    "loaded": done,
                    "total": total,
                    "status": "complete"
                }),
            );
        }

        Ok(config)
    }

    pub fn scan_instruments(&self) -> Result<Vec<PathBuf>, AudioError> {
        let directories =
            BasicFileOperations::list_directories(&self.instruments_dir).map_err(|e| {
                AudioError::InstrumentError(format!("Cannot read instruments dir: {}", e))
            })?;
        Ok(directories
            .into_iter()
            .filter(|e| e.join("instrument.json").exists())
            .collect())
    }
}

impl FileManager for InstrumentFileManagerImpl {
    type Item = InstrumentItem;
    type Error = StorageError;

    fn get_file(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<Self::Item, Self::Error>> + Send {
        async move {
            let instrument_path = PathBuf::from(path);
            BasicFileOperations::validate_instrument_structure(&instrument_path)?;
            Ok(BasicFileOperations::create_instrument_item(
                &instrument_path,
            )?)
        }
    }

    fn file_exists(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send {
        async move {
            let path = PathBuf::from(path);
            Ok(BasicFileOperations::directory_exists(&path))
        }
    }

    fn update_file(
        &self,
        path: &str,
        _item: &Self::Item,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move { BasicFileOperations::validate_instrument_structure(&PathBuf::from(path)) }
    }

    fn list_files(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<FileMetadata>, Self::Error>> + Send {
        async move {
            let directories = BasicFileOperations::list_directories(&self.instruments_dir)?;
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
    }

    fn create_file(
        &self,
        item: &Self::Item,
    ) -> impl std::future::Future<Output = Result<String, Self::Error>> + Send {
        async move {
            BasicFileOperations::create_directory(&item.path)?;
            BasicFileOperations::create_directory(&item.samples_dir)?;
            Ok(item.path.to_string_lossy().to_string())
        }
    }

    fn delete_file(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let path = PathBuf::from(path);
            BasicFileOperations::delete_directory(&path)
        }
    }

    fn validate_file(
        &self,
        item: &Self::Item,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move { BasicFileOperations::validate_instrument_structure(&item.path) }
    }
}

impl InstrumentFileManager for InstrumentFileManagerImpl {
    fn scan_instrument_directories(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<PathBuf>, StorageError>> + Send {
        async move {
            let directories = BasicFileOperations::list_directories(&self.instruments_dir)?;

            let mut valid_directories = Vec::new();
            for dir in directories {
                if BasicFileOperations::validate_instrument_structure(&dir).is_ok() {
                    valid_directories.push(dir);
                }
            }

            Ok(valid_directories)
        }
    }

    fn get_instrument_directory(
        &self,
        folder: &str,
    ) -> impl std::future::Future<Output = Result<InstrumentItem, StorageError>> + Send {
        async move {
            let instrument_path = BasicFileOperations::get_instrument_path(folder)?;
            self.get_file(&instrument_path.to_string_lossy()).await
        }
    }

    fn validate_instrument_structure(
        &self,
        path: &PathBuf,
    ) -> impl std::future::Future<Output = Result<(), StorageError>> + Send {
        async move { BasicFileOperations::validate_instrument_structure(path) }
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

    fn get_file(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<Self::Item, Self::Error>> + Send {
        async move {
            let song_path = PathBuf::from(path);
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
    }

    fn file_exists(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send {
        async move {
            let path = PathBuf::from(path);
            Ok(BasicFileOperations::file_exists(&path))
        }
    }

    fn update_file(
        &self,
        path: &str,
        _item: &Self::Item,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move { BasicFileOperations::validate_song_file(&PathBuf::from(path)) }
    }

    fn list_files(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<FileMetadata>, Self::Error>> + Send {
        async move {
            let files = BasicFileOperations::list_files(&self.songs_dir)?;
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
    }

    fn create_file(
        &self,
        item: &Self::Item,
    ) -> impl std::future::Future<Output = Result<String, Self::Error>> + Send {
        async move {
            BasicFileOperations::create_file(&item.path, "")?;
            Ok(item.path.to_string_lossy().to_string())
        }
    }

    fn validate_file(
        &self,
        item: &Self::Item,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move { BasicFileOperations::validate_song_file(&item.path) }
    }

    fn delete_file(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let path = PathBuf::from(path);
            BasicFileOperations::delete_file(&path)
        }
    }
}

impl SongFileManager for SongFileManagerImpl {
    fn scan_song_files(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<SongFile>, StorageError>> + Send {
        async move {
            let files = BasicFileOperations::list_files(&self.songs_dir)?;

            let mut song_files = Vec::new();
            for file in files {
                if let Some(metadata) = BasicFileOperations::create_song_item(&file) {
                    song_files.push(metadata);
                }
            }

            Ok(song_files)
        }
    }

    fn get_song_file(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<SongItem, StorageError>> + Send {
        async move { self.get_file(path).await }
    }

    fn validate_song_file(
        &self,
        path: &PathBuf,
    ) -> impl std::future::Future<Output = Result<(), StorageError>> + Send {
        async move { BasicFileOperations::validate_song_file(path) }
    }
}
