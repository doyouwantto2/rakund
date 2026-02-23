use serde::{Deserialize, Serialize};

use super::{contribution::Contribution, layer::LayerRangeInfo};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InstrumentInfoResponse {
    pub name: String,
    pub folder: String,
    pub layers: Vec<String>,
    pub layer_ranges: Vec<LayerRangeInfo>,
    pub format: String,
    pub settings: Vec<(String, String)>,
    pub contribution: Contribution,
}

impl InstrumentInfoResponse {
    pub fn from_config(config: &crate::setup::config::InstrumentConfig, folder: &str) -> Self {
        let mut layer_ranges: Vec<LayerRangeInfo> = config
            .general
            .layers
            .iter()
            .map(|(_, range): (&String, &LayerRangeInfo)| range.clone())
            .collect();
        layer_ranges.sort_by_key(|range| range.lovel);

        Self {
            name: config.instrument.clone(),
            folder: folder.to_string(),
            layers: config
                .layers()
                .iter()
                .map(|l: &String| l.to_uppercase())
                .collect(),
            layer_ranges,
            format: config.files_format().to_string(),
            settings: config
                .settings
                .values
                .iter()
                .map(|(k, v): (&String, &serde_json::Value)| (k.clone(), v.to_string()))
                .collect(),
            contribution: config.contribution.clone(),
        }
    }
}
