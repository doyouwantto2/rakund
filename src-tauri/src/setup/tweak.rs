use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Dynamic settings structure that can handle any instrument-specific settings
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    /// Dynamic settings as key-value pairs
    #[serde(flatten)]
    pub values: HashMap<String, String>,
}

impl Settings {
    /// Create a new empty settings object
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    /// Get a setting value as a string
    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }
    
    /// Get a setting value as f32
    pub fn get_f32(&self, key: &str) -> Option<f32> {
        self.values
            .get(key)
            .and_then(|s| s.parse().ok())
    }
    
    /// Get a setting value as i32
    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.values
            .get(key)
            .and_then(|s| s.parse().ok())
    }
    
    /// Get a setting value as bool
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.values
            .get(key)
            .and_then(|s| s.parse::<bool>().ok())
    }
    
    /// Set a setting value
    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }
    
    /// Check if a setting exists
    pub fn contains(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    
    /// Get all setting keys
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.values.keys()
    }
    
    /// Audio settings (only what's actually used)
    pub fn fast_release(&self) -> Option<f32> {
        self.get_f32("fast_release")
    }
    
    pub fn slow_release(&self) -> Option<f32> {
        self.get_f32("slow_release")
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert from raw JSON Value to Settings
impl From<Value> for Settings {
    fn from(value: Value) -> Self {
        let mut settings = Settings::new();
        
        if let Value::Object(map) = value {
            for (key, val) in map {
                if let Value::String(s) = val {
                    settings.values.insert(key, s);
                } else {
                    // Convert non-string values to strings
                    settings.values.insert(key, val.to_string());
                }
            }
        }
        
        settings
    }
}

/// Convert Settings to JSON Value
impl From<Settings> for Value {
    fn from(settings: Settings) -> Self {
        let mut map = serde_json::Map::new();
        for (key, value) in settings.values {
            map.insert(key, Value::String(value));
        }
        Value::Object(map)
    }
}
