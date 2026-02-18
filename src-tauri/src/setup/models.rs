use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use crate::error::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct SampleInfo {
    pub file: String,
    pub layer: String,
    pub min_vel: u8,
    pub max_vel: u8,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct KeyData {
    pub note: String,
    pub midi_note: u8,
    pub lokey: u8,
    pub hikey: u8,
    pub samples: Vec<SampleInfo>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InstrumentConfig {
    pub instrument: String,
    pub layers: Vec<String>,
    pub keys: HashMap<String, KeyData>,
    #[serde(skip)]
    pub samples_cache: HashMap<String, Arc<Vec<f32>>>,
}

#[derive(Debug, Serialize)]
pub struct InstrumentInfo {
    pub name: String,
    pub layers: Vec<String>,
}

pub trait Instrument: Send + Sync {
    fn name(&self) -> &str;
    fn config(&self) -> &InstrumentConfig;
    fn load_config(&mut self) -> Result<()>;
    fn get_sample_path(&self, filename: &str) -> String;
    fn get_velocity_for_layer(&self, layer: &str) -> u8;
    fn get_layers(&self) -> Vec<String>;
}
