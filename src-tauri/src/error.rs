use thiserror::Error;
use std::string::String;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("No output device found")]
    NoOutputDevice,
    
    #[error("Audio play stream error: {0}")]
    PlayStreamError(#[from] cpal::PlayStreamError),
    
    #[error("Audio stream error: {0}")]
    StreamError(String),
    
    #[error("Audio config error: {0}")]
    ConfigError(#[from] cpal::DefaultStreamConfigError),
    
    #[error("Audio build stream error: {0}")]
    BuildStreamError(#[from] cpal::BuildStreamError),
    
    #[error("Failed to decode FLAC file '{0}': {1}")]
    FlacDecodeError(String, String),
    
    #[error("Instrument error: {0}")]
    InstrumentError(String),
    
    #[error("Note {0} not found in piano configuration")]
    NoteNotFound(u8),
    
    #[error("No samples found for note {0}")]
    NoSamplesFound(u8),
    
    #[error("Failed to load layer '{1}' for instrument '{0}': {2}")]
    LayerLoadError(String, String, String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
}

pub type Result<T> = std::result::Result<T, AudioError>;