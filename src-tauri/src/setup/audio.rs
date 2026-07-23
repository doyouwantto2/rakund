use crate::engine::{cache, decoder, parser};
use crate::error::{AudioError, Result};
use crate::extra::sketch::instrument::release;
use crate::setup::config::InstrumentConfig;
use crate::state;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use tauri::Emitter;

const MAX_VOICES: usize = 64;

const CMD_QUEUE_DEPTH: usize = 512;

#[derive(Debug)]
pub enum AudioCommand {
    PlayNote {
        midi: u8,
        velocity: u8,
        data: Arc<Vec<f32>>,
        pitch_ratio: f32,
    },
    StopNote { midi: u8 },
}

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
    pub cmd_tx: SyncSender<AudioCommand>,
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

    let (cmd_tx, cmd_rx): (SyncSender<AudioCommand>, Receiver<AudioCommand>) =
        mpsc::sync_channel(CMD_QUEUE_DEPTH);

    let is_sustained = Arc::new(Mutex::new(false));
    let sustained_clone = Arc::clone(&is_sustained);

    let mut voices: Vec<Voice> = Vec::with_capacity(MAX_VOICES);
    let mut mix: Vec<f32> = Vec::new();

    let stream = device
        .build_output_stream(
            &config.into(),
            move |output: &mut [f32], _| {
                output.fill(0.0);

                let sustained = sustained_clone.try_lock().map(|g| *g).unwrap_or(false);

                while let Ok(cmd) = cmd_rx.try_recv() {
                    match cmd {
                        AudioCommand::PlayNote {
                            midi,
                            velocity,
                            data,
                            pitch_ratio,
                        } => {
                            for v in voices.iter_mut() {
                                if v.midi_note == midi && !v.is_releasing {
                                    v.is_releasing = true;
                                }
                            }
                            if voices.len() >= MAX_VOICES {
                                if let Some(idx) = voices.iter().position(|v| v.is_releasing) {
                                    voices.remove(idx);
                                } else {
                                    voices.remove(0);
                                }
                            }
                            voices.push(Voice {
                                data,
                                playhead: 0.0,
                                pitch_ratio,
                                midi_note: midi,
                                is_releasing: false,
                                volume: velocity as f32 / 127.0,
                            });
                        }
                        AudioCommand::StopNote { midi } => {
                            for v in voices.iter_mut() {
                                if v.midi_note == midi {
                                    v.is_releasing = true;
                                }
                            }
                        }
                    }
                }

                let fast = release::get_fast();
                let slow = release::get_slow();
                let num_frames = output.len() / channels;

                if mix.len() < num_frames {
                    mix.resize(num_frames, 0.0);
                }
                for s in mix[..num_frames].iter_mut() {
                    *s = 0.0;
                }

                for v in voices.iter_mut() {
                    for frame_idx in 0..num_frames {
                        let pos = v.playhead as usize;
                        if pos + 1 >= v.data.len() {
                            v.volume = 0.0;
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

                voices.retain(|v| v.volume > 0.001 && (v.playhead as usize + 1) < v.data.len());

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
            },
            |err| eprintln!("Audio stream error: {:?}", err),
            None,
        )
        .map_err(AudioError::BuildStreamError)?;

    stream.play().map_err(AudioError::PlayStreamError)?;
    Ok(AudioHandle {
        cmd_tx,
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

    let fast_release = config.fast_release().unwrap_or(0.9998);
    let slow_release = config.slow_release().unwrap_or(0.99999);
    release::set(fast_release, slow_release);

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
                            "loaded":   done,
                            "total":    total,
                            "status":   "loading"
                        }),
                    );
                }
            }
        }
    }

    if let Some(handle) = app {
        let _ = handle.emit(
            "load_progress",
            serde_json::json!({
                "progress": 100.0,
                "loaded":   done,
                "total":    total,
                "status":   "complete"
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
