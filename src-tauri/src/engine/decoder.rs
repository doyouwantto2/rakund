use claxon::FlacReader;
use std::sync::Arc;

pub fn decode_flac(path: &str) -> Arc<Vec<f32>> {
    let mut reader = FlacReader::open(path).expect("Failed to open FLAC");
    let spec = reader.streaminfo();
    let norm = 1.0 / (1 << (spec.bits_per_sample - 1)) as f32;

    let samples: Vec<f32> = reader
        .samples()
        .filter_map(Result::ok)
        .map(|s| s as f32 * norm)
        .collect();

    Arc::new(samples)
}
