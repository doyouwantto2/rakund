use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    #[serde(flatten)]
    pub values: HashMap<String, serde_json::Value>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.values.get(key).and_then(|v| match v {
            Value::String(s) => Some(s),
            _ => None,
        })
    }

    pub fn get_f32(&self, key: &str) -> Option<f32> {
        self.values.get(key).and_then(|v| match v {
            Value::Number(n) => n.as_f64().map(|f| f as f32),
            Value::String(s) => s.parse().ok(),
            _ => None,
        })
    }

    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.values.get(key).and_then(|v| match v {
            Value::Number(n) => n.as_i64().map(|i| i as i32),
            Value::String(s) => s.parse().ok(),
            _ => None,
        })
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.values.get(key).and_then(|v| match v {
            Value::Bool(b) => Some(*b),
            Value::String(s) => s.parse::<bool>().ok(),
            _ => None,
        })
    }

    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, Value::String(value));
    }

    pub fn set_f32(&mut self, key: String, value: f32) {
        self.values.insert(
            key,
            Value::Number(serde_json::Number::from_f64(value as f64).expect("Invalid f32 value")),
        );
    }

    pub fn set_i32(&mut self, key: String, value: i32) {
        self.values.insert(
            key,
            Value::Number(serde_json::Number::from_f64(value as f64).expect("Invalid i32 value")),
        );
    }

    pub fn set_bool(&mut self, key: String, value: bool) {
        self.values.insert(key, Value::Bool(value));
    }

    pub fn contains(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.values.keys()
    }

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

impl From<Value> for Settings {
    fn from(value: Value) -> Self {
        let mut settings = Settings::new();

        if let Value::Object(map) = value {
            for (key, val) in map {
                settings.values.insert(key, val);
            }
        }

        settings
    }
}

impl From<Settings> for Value {
    fn from(settings: Settings) -> Self {
        let mut map = serde_json::Map::new();
        for (key, value) in settings.values {
            map.insert(key, value);
        }
        Value::Object(map)
    }
}
