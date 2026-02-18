use crate::commands::player::SAMPLE_CACHE;
use crate::error::{AudioError, Result};
use crate::setup::models::InstrumentConfig;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::fs;
use std::sync::{Arc, Mutex};

const FAST_RELEASE: f32 = 0.99989;
const SLOW_RELEASE: f32 = 0.999982;

#[derive(Clone)]
pub struct Voice {
    pub data: Arc<Vec<f32>>,
    pub playhead: f32,
    pub pitch_ratio: f32,
    pub midi_note: u8,
    pub is_releasing: bool,
    pub volume: f32,
}

pub struct AudioHandle {
    pub active_voices: Arc<Mutex<Vec<Voice>>>,
    pub is_sustained: Arc<Mutex<bool>>,
    pub _stream: cpal::Stream,
}

pub fn start_stream() -> Result<AudioHandle> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or(AudioError::NoOutputDevice)?;
    let config = device.default_output_config()?;
    let channels = config.channels() as usize;

    let active_voices = Arc::new(Mutex::new(Vec::<Voice>::new()));
    let is_sustained = Arc::new(Mutex::new(false));
    let voices_clone = Arc::clone(&active_voices);
    let sustained_clone = Arc::clone(&is_sustained);

    let stream = device
        .build_output_stream(
            &config.into(),
            move |output: &mut [f32], _| {
                let mut voices = match voices_clone.try_lock() {
                    Ok(g) => g,
                    Err(_) => return,
                };
                let sustained = sustained_clone.try_lock().map(|g| *g).unwrap_or(false);

                for s in output.iter_mut() {
                    *s = 0.0;
                }

                let num_voices = voices.len() as f32;
                let gain = if num_voices > 1.0 {
                    1.0 / (num_voices.sqrt() * 1.2)
                } else {
                    0.7
                };
                let num_frames = output.len() / channels;

                for v in voices.iter_mut() {
                    for frame_idx in 0..num_frames {
                        let pos = v.playhead as usize;
                        if pos >= v.data.len() {
                            break;
                        }
                        let sample = v.data[pos] * gain * v.volume;
                        for ch in 0..channels {
                            let idx = frame_idx * channels + ch;
                            if idx < output.len() {
                                output[idx] = (output[idx] + sample).clamp(-1.0, 1.0);
                            }
                        }
                        if v.is_releasing {
                            v.volume *= if sustained {
                                SLOW_RELEASE
                            } else {
                                FAST_RELEASE
                            };
                        }
                        v.playhead += v.pitch_ratio;
                    }
                }
                voices.retain(|v| (v.playhead as usize) < v.data.len() && v.volume > 0.0005);
            },
            |err| eprintln!("Audio error: {:?}", err),
            None,
        )
        .map_err(AudioError::BuildStreamError)?;

    stream.play().map_err(AudioError::PlayStreamError)?;
    Ok(AudioHandle {
        active_voices,
        is_sustained,
        _stream: stream,
    })
}

pub fn initialize_audio() -> Result<()> {
    println!("[INIT] Loading FLAC samples into RAM (keyed by midi:layer)...");

    let cwd = std::env::current_dir().map_err(|e| AudioError::InstrumentError(e.to_string()))?;

    let samples_dir = cwd.join("data/instrument/Samples");
    let config_path = cwd.join("data/instrument.json");

    if !samples_dir.exists() {
        return Err(AudioError::InstrumentError(format!(
            "Samples directory not found: {:?}",
            samples_dir
        )));
    }

    // Parse instrument.json to get midi -> layer -> file mapping
    let raw = fs::read_to_string(&config_path)
        .map_err(|e| AudioError::InstrumentError(format!("Cannot read instrument.json: {}", e)))?;
    let config: InstrumentConfig = serde_json::from_str(&raw)
        .map_err(|e| AudioError::InstrumentError(format!("Invalid instrument.json: {}", e)))?;

    println!(
        "[INIT] Instrument: {} ({} keys)",
        config.instrument,
        config.keys.len()
    );

    // Sort keys numerically for clean log output
    let mut midi_keys: Vec<u8> = config.keys.keys().filter_map(|k| k.parse().ok()).collect();
    midi_keys.sort();

    let total = midi_keys.len() * config.layers.len();
    let mut done = 0;

    // File-level decode cache: avoid decoding the same FLAC twice
    // (multiple midi notes share one recorded sample, e.g. "PP C3.flac" covers C3..D3)
    let mut file_cache: std::collections::HashMap<String, Arc<Vec<f32>>> =
        std::collections::HashMap::new();

    for midi in &midi_keys {
        let key_data = &config.keys[&midi.to_string()];

        for sample_info in &key_data.samples {
            // Cache key: exact "{midi}:{layer}" — zero ambiguity, zero overlap
            let cache_key = format!("{}:{}", midi, sample_info.layer);

            // Decode each unique FLAC file once, then clone the Arc for all notes using it
            let file_key = sample_info.file.to_lowercase();
            let data = if let Some(cached) = file_cache.get(&file_key) {
                cached.clone()
            } else {
                // Find file case-insensitively on disk
                let path = find_flac_case_insensitive(&samples_dir, &sample_info.file).ok_or_else(
                    || AudioError::InstrumentError(format!("FLAC not found: {}", sample_info.file)),
                )?;
                let decoded = crate::engine::decoder::decode_flac(&path.to_string_lossy())?;
                file_cache.insert(file_key, decoded.clone());
                decoded
            };

            SAMPLE_CACHE.lock().unwrap().insert(cache_key, data);
            done += 1;

            if done % 50 == 0 || done == total {
                println!(
                    "[INIT] [{}/{}] {:.0}%",
                    done,
                    total,
                    (done as f32 / total as f32) * 100.0
                );
            }
        }
    }

    println!("[INIT] Done — {} entries in cache", done);
    Ok(())
}

fn find_flac_case_insensitive(dir: &std::path::Path, name: &str) -> Option<std::path::PathBuf> {
    let lower = name.to_lowercase();
    fs::read_dir(dir).ok()?.flatten().find_map(|e| {
        if e.file_name().to_string_lossy().to_lowercase() == lower {
            Some(e.path())
        } else {
            None
        }
    })
}
