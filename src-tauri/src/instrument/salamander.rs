use crate::engine::decoder;
use crate::engine::cache::SampleCache;
use crate::error::{AudioError, Result};
use crate::setup::models::{Instrument, InstrumentConfig};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

pub struct SalamanderInstrument {
    config: InstrumentConfig,
}

impl SalamanderInstrument {
    pub fn new() -> Result<Self> {
        let mut instrument = Self {
            config: InstrumentConfig {
                instrument: "salamander".to_string(),
                layers: (1..=16).map(|i| format!("V{:02}", i)).collect(),
                keys: HashMap::new(),
                samples_cache: HashMap::new(),
            },
        };
        instrument.load_config()?;
        Ok(instrument)
    }
}

impl Instrument for SalamanderInstrument {
    fn name(&self) -> &str {
        "salamander"
    }

    fn config(&self) -> &InstrumentConfig {
        &self.config
    }

    fn load_config(&mut self) -> Result<()> {
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let config_path = cwd.join("data").join("map").join("salamander.json");

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
        // Extract velocity from V01, V02, etc.
        if let Some(num_str) = layer.strip_prefix("V") {
            if let Ok(num) = num_str.parse::<u8>() {
                return match num {
                    1 => 20,
                    2 => 35,
                    3 => 50,
                    4 => 65,
                    5 => 80,
                    6 => 95,
                    7 => 110,
                    8 => 125,
                    9..=16 => 127,
                    _ => 100,
                };
            }
        }
        100
    }

    fn get_layers(&self) -> Vec<String> {
        (1..=16).map(|i| format!("V{:02}", i)).collect()
    }
}

// Global cache instance for salamander
lazy_static::lazy_static! {
    pub static ref SALAM_CACHE: SampleCache = SampleCache::new();
}

// Salamander-specific caching functions
pub async fn load_salamander_layer(layer: &str, config: &InstrumentConfig) -> Result<()> {
    println!("[SALAM_CACHE] Loading layer {}", layer);

    // Find all samples for this layer with exact matching for salamander
    let mut samples_to_load = Vec::new();
    for key_data in config.keys.values() {
        for sample in &key_data.samples {
            // Exact layer matching for salamander (V01, V02, etc.)
            if sample.layer == layer {
                samples_to_load.push(sample.file.clone());
            }
        }
    }

    // Remove duplicates
    samples_to_load.sort();
    samples_to_load.dedup();

    println!(
        "[SALAM_CACHE] Found {} unique samples to load",
        samples_to_load.len()
    );

    // Load each sample using the generic SampleCache
    for sample_file in samples_to_load {
        if let Err(e) = load_salamander_sample(&sample_file).await {
            println!("[SALAM_CACHE] Failed to load {}: {:?}", sample_file, e);
            return Err(e);
        }
    }

    println!("[SALAM_CACHE] Successfully loaded layer {}", layer);
    Ok(())
}

async fn load_salamander_sample(filename: &str) -> Result<()> {
    let file_path = format!("data/salamander/Samples/{}", filename);

    println!("[SALAM_CACHE] Looking for file: {}", file_path);
    println!(
        "[SALAM_CACHE] Current working directory: {:?}",
        std::env::current_dir()
    );

    let path = Path::new(&file_path);
    if !path.exists() {
        println!("[SALAM_CACHE] File does not exist at path: {}", file_path);
        return Err(AudioError::SampleNotFound(file_path));
    }

    // Load FLAC file using existing decoder
    let samples = decoder::decode_flac(&file_path)?;

    // Store in cache
    SALAM_CACHE
        .cache
        .lock()
        .unwrap()
        .insert(filename.to_string(), samples);

    println!("[SALAM_CACHE] Loaded {} from {}", filename, file_path);
    Ok(())
}

pub fn get_salamander_sample(filename: &str) -> Option<Arc<Vec<f32>>> {
    println!("[SALAM_CACHE] Looking for sample in cache: {}", filename);
    let sample = SALAM_CACHE.cache.lock().unwrap().get(filename).cloned();

    if sample.is_some() {
        println!(
            "[SALAM_CACHE] Found {} in cache with {} samples",
            filename,
            sample.as_ref().map(|s: &Arc<Vec<f32>>| s.len()).unwrap_or(0)
        );
    } else {
        println!(
            "[SALAM_CACHE] Sample {} NOT found in cache. Available keys: {:?}",
            filename,
            SALAM_CACHE.cache.lock().unwrap().keys().collect::<Vec<_>>()
        );
    }

    sample
}

pub fn is_salamander_layer_loaded(layer: &str) -> bool {
    SALAM_CACHE
        .loaded_instruments
        .lock()
        .unwrap()
        .get("salamander")
        .map(|layers: &Vec<String>| layers.contains(&layer.to_string()))
        .unwrap_or(false)
}
