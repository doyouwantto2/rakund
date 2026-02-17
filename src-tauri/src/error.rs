use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("No output device found")]
    NoOutputDevice,
    
    #[error("Failed to get audio config: {0}")]
    ConfigError(String),
    
    #[error("Audio stream error: {0}")]
    StreamError(String),
    
    #[error("Failed to decode FLAC file '{0}': {1}")]
    FlacDecodeError(String, String),
    
    #[error("Sample file not found: {0}")]
    SampleNotFound(String),
    
    #[error("Note {0} not found in piano configuration")]
    NoteNotFound(u8),
    
    #[error("No samples found for note {0}")]
    NoSamplesFound(u8),
    
    #[error("Failed to load layer '{1}' for instrument '{0}': {2}")]
    LayerLoadError(String, String, String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Instrument configuration error: {0}")]
    InstrumentError(String),
}

pub type Result<T> = std::result::Result<T, AudioError>;
