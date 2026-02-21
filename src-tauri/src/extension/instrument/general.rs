use serde::Deserialize;
use std::collections::HashMap;

use super::layer::LayerRange;

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    pub layers: HashMap<String, LayerRange>,
    pub files_format: String,
}
