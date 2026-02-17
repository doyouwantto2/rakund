use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
pub struct SampleInfo {
    pub file: String,
    pub layer: String,
    pub min_vel: u8,
    pub max_vel: u8,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct KeyData {
    pub note: String,
    pub midi_note: u8,
    pub lokey: u8,
    pub hikey: u8,
    pub samples: Vec<SampleInfo>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SplendidConfig {
    pub instrument: String,
    pub layers: Vec<String>,
    pub keys: HashMap<String, KeyData>,
    #[serde(skip)]
    pub samples_cache: HashMap<String, Arc<Vec<f32>>>,
}
