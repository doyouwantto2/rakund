use claxon::FlacReader;
use std::sync::Arc;

pub fn decode_flac(path: &str) -> Result<Arc<Vec<f32>>, String> {
    let mut reader = FlacReader::open(path)
        .map_err(|e| format!("Failed to open FLAC file '{}': {}", path, e))?;
    
    let spec = reader.streaminfo();
    let norm = 1.0 / (1 << (spec.bits_per_sample - 1)) as f32;

    let samples: Vec<f32> = reader
        .samples()
        .filter_map(Result::ok)
        .map(|s| s as f32 * norm)
        .collect();

    if samples.is_empty() {
        return Err(format!("No samples found in FLAC file: {}", path));
    }

    Ok(Arc::new(samples))
}
