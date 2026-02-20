use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct SampleInfo {
    pub path: String,
    pub layer: String,
    pub lovel: String,
    pub hivel: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeyData {
    pub note: String,
    pub midi: String,
    pub pitch: String,
    pub lokey: String,
    pub hikey: String,
    pub samples: Vec<SampleInfo>,
}

impl KeyData {
    pub fn midi_num(&self) -> u8 {
        self.midi.parse().unwrap_or(0)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Contribution {
    pub authors: Vec<String>,
    pub published_date: String,
    pub licenses: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    pub layers: Vec<String>,
    pub files_format: String,
    pub fast_release: String,
    pub slow_release: String,
}

impl General {
    pub fn fast_release_f32(&self) -> f32 {
        self.fast_release.parse().unwrap_or(0.9998)
    }
    pub fn slow_release_f32(&self) -> f32 {
        self.slow_release.parse().unwrap_or(0.99999)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct InstrumentConfig {
    pub instrument: String,
    pub contribution: Contribution,
    pub general: General,
    #[serde(deserialize_with = "deserialize_piano_keys")]
    pub piano_keys: HashMap<String, KeyData>,
}

impl InstrumentConfig {
    pub fn layers(&self) -> &Vec<String> {
        &self.general.layers
    }
    pub fn files_format(&self) -> &str {
        &self.general.files_format
    }
    pub fn fast_release(&self) -> f32 {
        self.general.fast_release_f32()
    }
    pub fn slow_release(&self) -> f32 {
        self.general.slow_release_f32()
    }
}

fn deserialize_piano_keys<'de, D>(
    deserializer: D,
) -> std::result::Result<HashMap<String, KeyData>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Vec<HashMap<String, KeyData>> = serde::Deserialize::deserialize(deserializer)?;
    let mut map = HashMap::new();
    for entry in raw {
        map.extend(entry);
    }
    Ok(map)
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppState {
    pub last_instrument: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct InstrumentInfo {
    pub name: String,
    pub folder: String,
    pub layers: Vec<String>,
    pub format: String,
}
