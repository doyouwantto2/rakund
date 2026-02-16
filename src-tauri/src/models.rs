use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
pub struct KeyRange {
    pub low: u8,
    pub high: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SampleInfo {
    pub file: String,
    #[serde(rename = "keyRange")]
    pub key_range: KeyRange,
    #[serde(rename = "velocityRange")]
    pub velocity_range: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeyData {
    #[serde(rename = "midiNote")]
    pub midi_note: u8,
    #[serde(rename = "noteName")]
    pub note_name: String,
    pub samples: Vec<SampleInfo>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SplendidConfig {
    pub keys: HashMap<String, KeyData>,
    // We tell Serde to ignore this during JSON parsing
    #[serde(skip)]
    pub samples_cache: HashMap<String, Arc<Vec<f32>>>,
}

pub struct LoadedSample {
    pub data: Arc<Vec<f32>>,
    pub name: String,
}
