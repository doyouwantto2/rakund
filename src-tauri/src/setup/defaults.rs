use crate::error::{AudioError, Result};

pub const DEFAULT_INSTRUMENT: &str = "splendid";
pub const DEFAULT_LAYER: &str = "MF";

pub fn get_default_instrument() -> String {
    DEFAULT_INSTRUMENT.to_string()
}

pub fn get_default_layer() -> String {
    DEFAULT_LAYER.to_string()
}

pub fn validate_instrument(instrument: &str) -> Result<()> {
    match instrument {
        "splendid" | "salamander" => Ok(()),
        _ => Err(AudioError::InstrumentError(format!(
            "Unknown instrument: {}",
            instrument
        ))),
    }
}

pub fn validate_layer(instrument: &str, layer: &str) -> Result<()> {
    validate_instrument(instrument)?;

    let valid_layers: Vec<String> = match instrument {
        "splendid" => vec!["PP", "MP", "MF", "FF"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        "salamander" => (1..=16).map(|i| format!("V{:02}", i)).collect::<Vec<_>>(),
        _ => {
            return Err(AudioError::InstrumentError(format!(
                "Unknown instrument: {}",
                instrument
            )))
        }
    };

    if valid_layers.contains(&layer.to_string()) {
        Ok(())
    } else {
        Err(AudioError::InstrumentError(format!(
            "Invalid layer '{}' for instrument '{}'",
            layer, instrument
        )))
    }
}
