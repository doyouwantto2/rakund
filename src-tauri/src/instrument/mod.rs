pub mod splendid;
pub mod salamander;

// Re-export the main instrument types
pub use splendid::SplendidInstrument;
pub use salamander::SalamanderInstrument;

// Re-export caching functions
pub use splendid::{load_splendid_layer, get_splendid_sample, is_splendid_layer_loaded};
pub use salamander::{load_salamander_layer, get_salamander_sample, is_salamander_layer_loaded};
