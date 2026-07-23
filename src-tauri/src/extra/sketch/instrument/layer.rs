use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LayerRangeInfo {
    pub name: String,
    pub lovel: u8,
    pub hivel: u8,
}

impl LayerRangeInfo {
    pub fn lovel_num(&self) -> u8 {
        self.lovel
    }

    pub fn hivel_num(&self) -> u8 {
        self.hivel
    }
}

pub fn deserialize_layers<'de, D>(
    deserializer: D,
) -> std::result::Result<HashMap<String, LayerRangeInfo>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct OldLayerFormat {
        lovel: u8,
        hivel: u8,
    }

    let raw: HashMap<String, serde_json::Value> = serde::Deserialize::deserialize(deserializer)?;
    let mut layers = HashMap::new();

    for (name, value) in raw {
        if let Ok(mut layer_info) = serde_json::from_value::<LayerRangeInfo>(value.clone()) {
            if layer_info.name.is_empty() {
                layer_info.name = name.clone();
            }
            layers.insert(name, layer_info);
        } else {
            if let Ok(old_layer) = serde_json::from_value::<OldLayerFormat>(value) {
                layers.insert(
                    name.clone(),
                    LayerRangeInfo {
                        name,
                        lovel: old_layer.lovel,
                        hivel: old_layer.hivel,
                    },
                );
            }
        }
    }

    Ok(layers)
}
