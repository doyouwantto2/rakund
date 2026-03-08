pub mod basic;
pub mod garbage;
pub mod handler;
pub mod items;
pub mod logic;

// Re-export commonly used types
pub use items::{
    InstrumentItem, SongItem, FileMetadata, FileType, SongFileType,
    FileManager, InstrumentFileManager, SongFileManager,
    InstrumentFileResponse, SongFileResponse, StorageError, StorageErrorType
};

pub use logic::{
    InstrumentFileManagerImpl, SongFileManagerImpl,
    CURRENT_INSTRUMENT, CURRENT_FOLDER
};

pub use basic::BasicFileOperations;

pub use handler::{FileHandler, FileManager as FileHandlerManager};
