use serde::Deserialize;
use std::collections::HashMap;

use super::layer::{LayerRangeInfo, deserialize_layers};

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    #[serde(deserialize_with = "deserialize_layers")]
    pub layers: HashMap<String, LayerRangeInfo>,
    pub files_format: String,
}
