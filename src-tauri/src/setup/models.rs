use crate::engine::optional::settings::Settings;
use crate::extension::instrument::{
    contribution::Contribution, general::General, layer::LayerRange, sample::KeyData,
    deserialize::deserialize_piano_keys,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppState {
    pub last_instrument: Option<String>,
}
