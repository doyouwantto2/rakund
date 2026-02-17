use crate::engine::decoder;
use crate::engine::cache::SampleCache;
use crate::error::{AudioError, Result};
use crate::setup::models::{Instrument, InstrumentConfig};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

pub struct SplendidInstrument {
    config: InstrumentConfig,
}

impl SplendidInstrument {
    pub fn new() -> Result<Self> {
        let mut instrument = Self {
            config: InstrumentConfig {
                instrument: "splendid".to_string(),
                layers: vec![
                    "PP".to_string(),
                    "MP".to_string(),
                    "MF".to_string(),
                    "FF".to_string(),
                ],
                keys: HashMap::new(),
                samples_cache: HashMap::new(),
            },
        };
        instrument.load_config()?;
        Ok(instrument)
    }
}

impl Instrument for SplendidInstrument {
    fn name(&self) -> &str {
        "splendid"
    }

    fn config(&self) -> &InstrumentConfig {
        &self.config
    }

    fn load_config(&mut self) -> Result<()> {
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let config_path = cwd.join("data").join("map").join("splendid.json");

        let config_data = fs::read_to_string(&config_path).map_err(|e| {
            AudioError::InstrumentError(format!("Could not find JSON at {:?}: {}", config_path, e))
        })?;

        let config: InstrumentConfig = serde_json::from_str(&config_data)
            .map_err(|e| AudioError::InstrumentError(format!("Invalid JSON: {}", e)))?;

        self.config = config;
        Ok(())
    }

    fn get_sample_path(&self, filename: &str) -> String {
        format!("data/{}/samples/{}", self.name(), filename)
    }

    fn get_velocity_for_layer(&self, layer: &str) -> u8 {
        match layer {
            "PP" => 20,  // Very soft (1-40 range)
            "MP" => 54,  // Medium soft (41-67 range)
            "MF" => 76,  // Medium hard (68-84 range)
            "FF" => 106, // Very hard (85-127 range)
            _ => 100,
        }
    }

    fn get_layers(&self) -> Vec<String> {
        self.config.layers.clone()
    }
}

// Global cache instance
lazy_static::lazy_static! {
    pub static ref SPLD_CACHE: SampleCache = SampleCache::new();
}

// Splendid-specific caching functions
pub async fn load_splendid_layer(layer: &str, config: &InstrumentConfig) -> Result<()> {
    println!("[SPLD_CACHE] Loading layer {}", layer);

    // Find all samples for this layer with case-sensitive matching for splendid
    let mut samples_to_load = Vec::new();
    for key_data in config.keys.values() {
        for sample in &key_data.samples {
            // Exact layer matching for splendid (JSON uses uppercase, files may be mixed case)
            if sample.layer == layer {
                samples_to_load.push(sample.file.clone());
            }
        }
    }

    // Remove duplicates
    samples_to_load.sort();
    samples_to_load.dedup();

    println!(
        "[SPLD_CACHE] Found {} unique samples to load",
        samples_to_load.len()
    );

    // Load each sample using the generic SampleCache
    for sample_file in samples_to_load {
        if let Err(e) = load_splendid_sample(&sample_file).await {
            println!("[SPLD_CACHE] Failed to load {}: {:?}", sample_file, e);
            return Err(e);
        }
    }

    // Mark this layer as loaded AFTER samples are actually loaded
    {
        let mut loaded_instruments = SPLD_CACHE.loaded_instruments.lock().unwrap();
        loaded_instruments
            .entry("splendid".to_string())
            .or_insert_with(Vec::new)
            .push(layer.to_string());
    }

    println!("[SPLD_CACHE] Successfully loaded layer {}", layer);
    Ok(())
}

async fn load_splendid_sample(filename: &str) -> Result<()> {
    let file_path = format!("data/splendid/Samples/{}", filename);

    println!("[SPLD_CACHE] Looking for file: {}", file_path);
    println!(
        "[SPLD_CACHE] Current working directory: {:?}",
        std::env::current_dir()
    );

    let path = Path::new(&file_path);
    if !path.exists() {
        println!("[SPLD_CACHE] File does not exist at path: {}", file_path);
        return Err(AudioError::SampleNotFound(file_path));
    }

    // Load FLAC file using existing decoder
    let samples = decoder::decode_flac(&file_path)?;

    // Store in cache
    SPLD_CACHE
        .cache
        .lock()
        .unwrap()
        .insert(filename.to_string(), samples);

    println!("[SPLD_CACHE] Loaded {} from {}", filename, file_path);
    Ok(())
}

pub fn get_splendid_sample(filename: &str) -> Option<Arc<Vec<f32>>> {
    println!("[SPLD_CACHE] Looking for sample in cache: {}", filename);
    let sample = SPLD_CACHE.cache.lock().unwrap().get(filename).cloned();

    if sample.is_some() {
        println!(
            "[SPLD_CACHE] Found {} in cache with {} samples",
            filename,
            sample.as_ref().map(|s: &Arc<Vec<f32>>| s.len()).unwrap_or(0)
        );
    } else {
        println!(
            "[SPLD_CACHE] Sample {} NOT found in cache. Available keys: {:?}",
            filename,
            SPLD_CACHE.cache.lock().unwrap().keys().collect::<Vec<_>>()
        );
    }

    sample
}

pub fn is_splendid_layer_loaded(layer: &str) -> bool {
    let loaded_instruments = SPLD_CACHE.loaded_instruments.lock().unwrap();

    // Use .map_or to handle the Option safely without creating temporary references
    let is_loaded = loaded_instruments
        .get("splendid")
        .map_or(false, |layers: &Vec<String>| layers.contains(&layer.to_string()));

    println!(
        "[DEBUG] Checking if layer {} is loaded: {}",
        layer, is_loaded
    );
    is_loaded
}
