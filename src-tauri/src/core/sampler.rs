use crate::error::AudioError;
use crate::setup::config::{AppState, InstrumentConfig};
use crate::state;
use crate::storage::handler::FileHandler;
use tauri::{AppHandle, State};

#[tauri::command]
pub fn get_available_instruments(
) -> Result<Vec<crate::extra::sketch::instrument::response::InstrumentInfoResponse>, String> {
    let file_handler = FileHandler::new().map_err(|e| e.to_string())?;

    let directories = file_handler.scan_instrument_directories().map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for (_index, folder) in directories.iter().enumerate() {
        let folder_name = folder
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        // Read instrument.json to get full instrument data
        let json_path = folder.join("instrument.json");
        let raw = crate::storage::basic::BasicFileOperations::read_file_content(&json_path)
            .map_err(|e| e.to_string())?;

        if raw.is_empty() {
            continue;
        }

        match serde_json::from_str::<InstrumentConfig>(&raw) {
            Ok(config) => {
                let info =
                    crate::extra::sketch::instrument::response::InstrumentInfoResponse::from_config(
                        &config,
                        folder_name,
                    );
                result.push(info);
            }
            Err(e) => {
                eprintln!("[SCAN] Failed to parse JSON at {:?}: {}", json_path, e);
            }
        }
    }

    Ok(result)
}

// Backward compatibility alias
#[tauri::command]
pub async fn get_available_instruments_files(
) -> Result<Vec<crate::extra::sketch::instrument::response::InstrumentInfoResponse>, String> {
    // Just call the main function
    get_available_instruments()
}

#[tauri::command]
pub async fn load_instrument(
    folder: String,
    _app: AppHandle,
    _state: State<'_, AppState>,
) -> Result<crate::extra::sketch::instrument::response::InstrumentInfoResponse, String> {
    // Use FileHandler for all operations
    let file_handler = FileHandler::new().map_err(|e| e.to_string())?;

    // Validate instrument exists using FileHandler
    let instrument_exists = file_handler
        .instrument_exists(&folder)
        .map_err(|e| e.to_string())?;

    if !instrument_exists {
        return Err(format!("Instrument '{}' does not exist", folder));
    }

    // For now, just return the basic info without loading
    let info = crate::extra::sketch::instrument::response::InstrumentInfoResponse::from_config(
        &crate::setup::config::InstrumentConfig::migrate_from_old("{}").unwrap(), &folder,
    );

    state::set_last_instrument(&folder).map_err(|e: AudioError| e.to_string())?;
    Ok(info)
}

#[tauri::command]
pub async fn get_app_state() -> Result<AppState, String> {
    state::read().map_err(|e: AudioError| e.to_string())
}

#[tauri::command]
pub async fn clear_last_instrument() -> Result<(), String> {
    state::clear_last_instrument().map_err(|e: AudioError| e.to_string())
}
