use crate::engine::{cache, decoder, parser};
use crate::error::{AudioError, Result};
use crate::external::sketch::instrument::release;
use crate::setup::config::InstrumentConfig;
use crate::setup::state;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::Emitter;

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

                let fast = release::get_fast();
                let slow = release::get_slow();
                let num_frames = output.len() / channels;
                let mut mix = vec![0.0f32; num_frames];

                for v in voices.iter_mut() {
                    for frame_idx in 0..num_frames {
                        let pos = v.playhead as usize;
                        if pos + 1 >= v.data.len() {
                            break;
                        }
                        let frac = v.playhead - pos as f32;
                        let sample = v.data[pos] * (1.0 - frac) + v.data[pos + 1] * frac;
                        mix[frame_idx] += sample * v.volume;
                        if v.is_releasing {
                            v.volume *= if sustained { slow } else { fast };
                        }
                        v.playhead += v.pitch_ratio;
                    }
                }

                let num_voices = voices.len().max(1) as f32;
                let gain = (1.0 / num_voices.sqrt()).min(1.0) * 0.8;

                for frame_idx in 0..num_frames {
                    let s = (mix[frame_idx] * gain).tanh();
                    for ch in 0..channels {
                        let idx = frame_idx * channels + ch;
                        if idx < output.len() {
                            output[idx] = s;
                        }
                    }
                }

                voices.retain(|v| (v.playhead as usize + 1) < v.data.len() && v.volume > 0.0005);
            },
            |err| eprintln!("Audio stream error: {:?}", err),
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

pub fn load_instrument(folder: &str) -> Result<InstrumentConfig> {
    let instrument_dir = state::instruments_dir()?.join(folder);
    load_instrument_from_path(&instrument_dir, None::<&tauri::AppHandle>)
}

pub fn load_instrument_with_progress(
    folder: &str,
    app: &tauri::AppHandle,
) -> Result<InstrumentConfig> {
    let instrument_dir = state::instruments_dir()?.join(folder);
    load_instrument_from_path(&instrument_dir, Some(app))
}

fn load_instrument_from_path(
    instrument_dir: &Path,
    app: Option<&tauri::AppHandle>,
) -> Result<InstrumentConfig> {
    let json_path = instrument_dir.join("instrument.json");

    let raw = fs::read_to_string(&json_path)
        .map_err(|e| AudioError::InstrumentError(format!("Cannot read instrument.json: {}", e)))?;

    let config = InstrumentConfig::migrate_from_old(&raw)
        .map_err(|e| AudioError::InstrumentError(format!("Invalid instrument.json: {}", e)))?;

    let fast_release = config.fast_release().unwrap_or_else(|| {
        println!("[LOAD] Using default fast_release (not specified in JSON)");
        0.9998
    });
    let slow_release = config.slow_release().unwrap_or_else(|| {
        println!("[LOAD] Using default slow_release (not specified in JSON)");
        0.99999
    });

    release::set(fast_release, slow_release);

    println!(
        "[LOAD] {} — format: {} — {} keys — layers: {:?}",
        config.instrument,
        config.files_format(),
        config.piano_keys.len(),
        config.layers(),
    );

    cache::clear();

    let mut midi_keys: Vec<u8> = config
        .piano_keys
        .keys()
        .filter_map(|k| k.parse().ok())
        .collect();
    midi_keys.sort();

    let total = midi_keys
        .iter()
        .map(|m| {
            config
                .piano_keys
                .get(&m.to_string())
                .map(|k| k.samples.len())
                .unwrap_or(0)
        })
        .sum::<usize>();
    let mut done = 0usize;
    let mut last_emitted_pct = -1i32;

    let mut file_cache: HashMap<String, Arc<Vec<f32>>> = HashMap::new();

    for midi in &midi_keys {
        let key_data = &config.piano_keys[&midi.to_string()];

        for (sample_idx, sample_info) in key_data.samples.iter().enumerate() {
            let sample_path = instrument_dir.join(&sample_info.path);
            let file_key = sample_path.to_string_lossy().to_lowercase();

            let data = if let Some(cached) = file_cache.get(&file_key) {
                cached.clone()
            } else {
                let decoded = decoder::decode(&sample_path.to_string_lossy())?;
                file_cache.insert(file_key, decoded.clone());
                decoded
            };

            cache::insert_by_index(*midi, sample_idx, data);
            done += 1;

            if let Some(handle) = app {
                let pct = ((done as f32 / total as f32) * 100.0) as i32;
                if pct != last_emitted_pct {
                    last_emitted_pct = pct;
                    let _ = handle.emit(
                        "load_progress",
                        serde_json::json!({
                            "progress": pct as f32,
                            "loaded": done,
                            "total": total,
                            "status": "loading"
                        }),
                    );
                }
            }
        }
    }

    println!("[LOAD] Done — {} entries cached", done);

    if let Some(handle) = app {
        let _ = handle.emit(
            "load_progress",
            serde_json::json!({
                "progress": 100.0,
                "loaded": done,
                "total": total,
                "status": "done"
            }),
        );
    }

    Ok(config)
}

pub fn scan_instruments() -> Result<Vec<PathBuf>> {
    let dir = state::instruments_dir()?;
    let entries = fs::read_dir(&dir)
        .map_err(|e| AudioError::InstrumentError(format!("Cannot read instruments dir: {}", e)))?;

    Ok(entries
        .flatten()
        .filter(|e| e.path().is_dir())
        .filter(|e| e.path().join("instrument.json").exists())
        .map(|e| e.path())
        .collect())
}

pub fn pitch_ratio(recorded_midi: u8, target_midi: u8) -> f32 {
    2.0f32.powf((target_midi as f32 - recorded_midi as f32) / 12.0)
}

pub fn pitch_to_midi(pitch: &str) -> Option<u8> {
    parser::note_name_to_midi(pitch)
}
