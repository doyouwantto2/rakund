use crate::setup::tweak::Settings;
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Contribution {
    pub authors: Vec<String>,
    pub published_date: String,
    pub licenses: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    pub layers: Vec<String>,
    pub files_format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InstrumentConfig {
    pub instrument: String,
    pub contribution: Contribution,
    pub general: General,
    pub settings: Settings,
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
    pub fn fast_release(&self) -> Option<f32> {
        self.settings.fast_release()
    }
    pub fn slow_release(&self) -> Option<f32> {
        self.settings.slow_release()
    }
    pub fn get_setting(&self, key: &str) -> Option<&String> {
        self.settings.get_string(key)
    }

    pub fn migrate_from_old(json_str: &str) -> Result<Self, serde_json::Error> {
        if let Ok(new_config) = serde_json::from_str::<Self>(json_str) {
            return Ok(new_config);
        }

        #[derive(Deserialize)]
        struct OldGeneral {
            layers: Vec<String>,
            files_format: String,
            fast_release: String,
            slow_release: String,
        }

        #[derive(Deserialize)]
        struct OldConfig {
            instrument: String,
            contribution: Contribution,
            general: OldGeneral,
            #[serde(deserialize_with = "deserialize_piano_keys")]
            piano_keys: HashMap<String, KeyData>,
        }

        let old_config: OldConfig = serde_json::from_str(json_str)?;

        let mut settings = Settings::new();
        settings.set("fast_release".to_string(), old_config.general.fast_release);
        settings.set("slow_release".to_string(), old_config.general.slow_release);

        Ok(Self {
            instrument: old_config.instrument,
            contribution: old_config.contribution,
            general: General {
                layers: old_config.general.layers,
                files_format: old_config.general.files_format,
            },
            settings,
            piano_keys: old_config.piano_keys,
        })
    }
}

fn deserialize_piano_keys<'de, D>(
    deserializer: D,
) -> std::result::Result<HashMap<String, KeyData>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Vec<serde_json::Value> = serde::Deserialize::deserialize(deserializer)?;
    let mut map = HashMap::new();

    for entry in raw {
        if let serde_json::Value::Object(obj) = entry {
            for (midi_key, key_data) in obj {
                if let Ok(key_data) = serde_json::from_value::<KeyData>(key_data) {
                    map.insert(midi_key, key_data);
                }
            }
        }
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
    pub settings: Vec<(String, String)>,
    pub contribution: Contribution,
}

impl InstrumentInfo {
    pub fn from_config(config: &crate::setup::models::InstrumentConfig, folder: &str) -> Self {
        Self {
            name: config.instrument.clone(),
            folder: folder.to_string(),
            layers: config.layers().iter().map(|l| l.to_uppercase()).collect(),
            format: config.files_format().to_string(),
            settings: config.settings.values.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            contribution: config.contribution.clone(),
        }
    }
}
