use crate::setup::tweak::Settings;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct LayerRange {
    pub lovel: u8,
    pub hivel: u8,
}

impl LayerRange {
    pub fn lovel_num(&self) -> u8 {
        self.lovel
    }

    pub fn hivel_num(&self) -> u8 {
        self.hivel
    }
}

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Contribution {
    pub authors: Vec<String>,
    pub published_date: String,
    pub licenses: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    pub layers: std::collections::HashMap<String, LayerRange>,
    pub files_format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InstrumentConfig {
    pub instrument: String,
    pub description: Option<String>,
    pub contribution: Contribution,
    pub general: General,
    pub settings: Settings,
    #[serde(deserialize_with = "deserialize_piano_keys")]
    pub piano_keys: HashMap<String, KeyData>,
}

impl InstrumentConfig {
    pub fn layers(&self) -> Vec<String> {
        let mut layers: Vec<(String, u8)> = self
            .general
            .layers
            .iter()
            .map(|(name, range)| (name.clone(), range.lovel_num()))
            .collect();
        layers.sort_by_key(|(_, lovel)| *lovel);
        layers.into_iter().map(|(name, _)| name).collect()
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
            layers: std::collections::HashMap<String, OldLayerRange>,
            files_format: String,
            fast_release: String,
            slow_release: String,
        }

        #[derive(Deserialize)]
        struct OldLayerRange {
            lovel: u8,
            hivel: u8,
        }

        #[derive(Deserialize)]
        struct OldConfig {
            instrument: String,
            contribution: Contribution,
            general: OldGeneral,
            piano_keys: HashMap<String, KeyData>,
        }

        let old_config: OldConfig = serde_json::from_str(json_str)?;

        let mut settings = Settings::new();

        if let Ok(fast_release) = old_config.general.fast_release.parse::<f32>() {
            settings.set_f32("fast_release".to_string(), fast_release);
        } else {
            settings.set("fast_release".to_string(), old_config.general.fast_release);
        }

        if let Ok(slow_release) = old_config.general.slow_release.parse::<f32>() {
            settings.set_f32("slow_release".to_string(), slow_release);
        } else {
            settings.set("slow_release".to_string(), old_config.general.slow_release);
        }

        let mut layers = std::collections::HashMap::new();
        for (layer_name, layer_range) in &old_config.general.layers {
            layers.insert(
                layer_name.clone(),
                LayerRange {
                    lovel: layer_range.lovel,
                    hivel: layer_range.hivel,
                },
            );
        }

        Ok(Self {
            instrument: old_config.instrument,
            description: None,
            contribution: old_config.contribution,
            general: General {
                layers,
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
    // Deserialize as array of objects first
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
    pub layer_ranges: Vec<LayerRangeInfo>,
    pub format: String,
    pub settings: Vec<(String, String)>,
    pub contribution: Contribution,
}

#[derive(Debug, Serialize, Clone)]
pub struct LayerRangeInfo {
    pub name: String,
    pub lovel: u8,
    pub hivel: u8,
}

impl InstrumentInfo {
    pub fn from_config(config: &crate::setup::models::InstrumentConfig, folder: &str) -> Self {
        let mut layer_ranges: Vec<LayerRangeInfo> = config
            .general
            .layers
            .iter()
            .map(|(name, range)| LayerRangeInfo {
                name: name.clone(),
                lovel: range.lovel_num(),
                hivel: range.hivel_num(),
            })
            .collect();
        layer_ranges.sort_by_key(|range| range.lovel);

        Self {
            name: config.instrument.clone(),
            folder: folder.to_string(),
            layers: config.layers().iter().map(|l| l.to_uppercase()).collect(),
            layer_ranges,
            format: config.files_format().to_string(),
            settings: config
                .settings
                .values
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
            contribution: config.contribution.clone(),
        }
    }
}
