use crate::error::{AudioError, Result};
use crate::setup::config::AppState;
use std::fs;
use std::path::PathBuf;

fn state_path() -> Result<PathBuf> {
    let base = dirs_next::data_dir()
        .ok_or_else(|| AudioError::InstrumentError("Cannot find data directory".to_string()))?;
    Ok(base.join("rakund").join("state.json"))
}

pub fn instruments_dir() -> Result<PathBuf> {
    let base = dirs_next::config_dir()
        .ok_or_else(|| AudioError::InstrumentError("Cannot find config directory".to_string()))?;
    let dir = base.join("rakund").join("instruments");
    fs::create_dir_all(&dir).map_err(|e| {
        AudioError::InstrumentError(format!("Cannot create instruments dir: {}", e))
    })?;
    Ok(dir)
}

pub fn read() -> Result<AppState> {
    let path = state_path()?;
    if !path.exists() {
        return Ok(AppState::default());
    }
    let raw = fs::read_to_string(&path)
        .map_err(|e| AudioError::InstrumentError(format!("Cannot read state: {}", e)))?;
    serde_json::from_str(&raw)
        .map_err(|e| AudioError::InstrumentError(format!("Invalid state.json: {}", e)))
}

pub fn write(state: &AppState) -> Result<()> {
    let path = state_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AudioError::InstrumentError(format!("Cannot create state dir: {}", e)))?;
    }
    let raw = serde_json::to_string_pretty(state)
        .map_err(|e| AudioError::InstrumentError(format!("Cannot serialize state: {}", e)))?;
    fs::write(&path, raw)
        .map_err(|e| AudioError::InstrumentError(format!("Cannot write state: {}", e)))
}

pub fn set_last_instrument(folder: &str) -> Result<()> {
    let mut state = read()?;
    state.last_instrument = Some(folder.to_string());
    write(&state)
}

pub fn clear_last_instrument() -> Result<()> {
    let mut state = read()?;
    state.last_instrument = None;
    write(&state)
}
