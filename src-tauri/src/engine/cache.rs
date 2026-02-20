use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    pub static ref SAMPLE_CACHE: Arc<Mutex<HashMap<String, Arc<Vec<f32>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

fn key_by_index(midi: u8, layer_idx: usize) -> String {
    format!("{}:{}", midi, layer_idx)
}

pub fn insert_by_index(midi: u8, layer_idx: usize, data: Arc<Vec<f32>>) {
    SAMPLE_CACHE
        .lock()
        .unwrap()
        .insert(key_by_index(midi, layer_idx), data);
}

pub fn get_by_index(midi: u8, layer_idx: usize) -> Option<Arc<Vec<f32>>> {
    SAMPLE_CACHE
        .lock()
        .unwrap()
        .get(&key_by_index(midi, layer_idx))
        .cloned()
}

pub fn clear() {
    SAMPLE_CACHE.lock().unwrap().clear();
}
