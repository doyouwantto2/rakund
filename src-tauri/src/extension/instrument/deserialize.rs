use std::collections::HashMap;

use super::sample::KeyData;

pub fn deserialize_piano_keys<'de, D>(
    deserializer: D,
) -> std::result::Result<HashMap<String, KeyData>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Vec<serde_json::Value> = serde::Deserialize::deserialize(deserializer)?;
    let mut map = HashMap::new();

    for entry in raw {
        if let serde_json::Value::Object(obj) = entry {
            for (midi_key, key_data) in obj {
                if let Ok(key_data) = serde_json::from_value::<KeyData>(key_data) {
                    map.insert(midi_key, key_data);
                }
            }
        }
    }

    Ok(map)
}
