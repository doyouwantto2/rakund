use claxon::FlacReader;
use std::sync::Arc;
use crate::error::{AudioError, Result};

pub fn decode_flac(path: &str) -> Result<Arc<Vec<f32>>> {
    let mut reader =
        FlacReader::open(path).map_err(|e| AudioError::FlacDecodeError(path.to_string(), e.to_string()))?;

    let spec = reader.streaminfo();
    let norm = 1.0 / (1 << (spec.bits_per_sample - 1)) as f32;

    let samples: std::result::Result<Vec<f32>, _> = reader
        .samples()
        .map(|s_res| s_res.map(|s| s as f32 * norm))
        .collect();

    let decoded_samples = samples.map_err(|e| AudioError::FlacDecodeError(path.to_string(), e.to_string()))?;

    if decoded_samples.is_empty() {
        return Err(AudioError::FlacDecodeError(path.to_string(), "No audio data found".to_string()));
    }

    Ok(Arc::new(decoded_samples))
}
