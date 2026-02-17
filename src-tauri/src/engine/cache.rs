use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SampleCache {
    pub cache: Arc<Mutex<HashMap<String, Arc<Vec<f32>>>>>,
    pub loaded_instruments: Arc<Mutex<HashMap<String, Vec<String>>>>, // instrument -> loaded layers
}

impl SampleCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            loaded_instruments: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_sample(&self, filename: &str) -> Option<Arc<Vec<f32>>> {
        let cache = self.cache.lock().unwrap();
        cache.get(filename).cloned()
    }

    pub fn set_sample(&self, filename: &str, samples: Arc<Vec<f32>>) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(filename.to_string(), samples);
    }

    pub fn get_loaded_layers(&self, instrument: &str) -> Vec<String> {
        let loaded_instruments = self.loaded_instruments.lock().unwrap();
        loaded_instruments.get(instrument).cloned().unwrap_or_default()
    }

    pub fn set_layer_loaded(&self, instrument: &str, layer: &str) {
        let mut loaded_instruments = self.loaded_instruments.lock().unwrap();
        loaded_instruments
            .entry(instrument.to_string())
            .or_insert_with(|| Vec::new())
            .push(layer.to_string());
    }
}
