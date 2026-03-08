use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SampleInfo {
    pub path: String,
    pub layer: String,
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
