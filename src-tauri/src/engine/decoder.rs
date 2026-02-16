use claxon::FlacReader;
use std::sync::Arc;

pub fn decode_flac(path: &str) -> Result<Arc<Vec<f32>>, String> {
    let mut reader =
        FlacReader::open(path).map_err(|e| format!("Claxon error opening '{}': {}", path, e))?;

    let spec = reader.streaminfo();
    let norm = 1.0 / (1 << (spec.bits_per_sample - 1)) as f32;

    let samples: Result<Vec<f32>, _> = reader
        .samples()
        .map(|s_res| s_res.map(|s| s as f32 * norm))
        .collect();

    let decoded_samples = samples.map_err(|e| format!("Decoding error in '{}': {}", path, e))?;

    if decoded_samples.is_empty() {
        return Err(format!("No audio data found in: {}", path));
    }

    Ok(Arc::new(decoded_samples))
}
