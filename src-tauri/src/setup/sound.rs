use crate::commands::player::SAMPLE_CACHE;
use crate::error::{AudioError, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::fs;
use std::sync::{Arc, Mutex};

const FAST_RELEASE: f32 = 0.9998;
const SLOW_RELEASE: f32 = 0.99999;

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
    let config = device
        .default_output_config()
        .map_err(|e| AudioError::ConfigError(e))?;

    let channels = config.channels() as usize;
    let config_clone = config.clone();

    let active_voices = Arc::new(Mutex::new(Vec::<Voice>::new()));
    let is_sustained = Arc::new(Mutex::new(false));

    let voices_clone = Arc::clone(&active_voices);
    let sustained_clone = Arc::clone(&is_sustained);

    let stream = device
        .build_output_stream(
            &config_clone.into(),
            move |output: &mut [f32], _| {
                let mut voices = match voices_clone.try_lock() {
                    Ok(guard) => guard,
                    Err(_) => return,
                };

                let sustained = sustained_clone.try_lock().map(|g| *g).unwrap_or(false);

                // Zero out the output buffer first
                for sample in output.iter_mut() {
                    *sample = 0.0;
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

                        // Write same sample to all channels
                        for ch in 0..channels {
                            let out_idx = frame_idx * channels + ch;
                            if out_idx < output.len() {
                                output[out_idx] = (output[out_idx] + sample).clamp(-1.0, 1.0);
                            }
                        }

                        // Only decay after key release — key held = full volume, no decay
                        if v.is_releasing {
                            let decay = if sustained {
                                SLOW_RELEASE
                            } else {
                                FAST_RELEASE
                            };
                            v.volume *= decay;
                        }

                        v.playhead += v.pitch_ratio;
                    }
                }

                // Remove finished or fully faded voices
                voices.retain(|v| (v.playhead as usize) < v.data.len() && v.volume > 0.0005);
            },
            |err| {
                eprintln!(
                    "Audio error: {:?}",
                    AudioError::StreamError(format!("{:?}", err))
                )
            },
            None,
        )
        .map_err(|e| AudioError::BuildStreamError(e))?;

    stream.play().map_err(|e| AudioError::PlayStreamError(e))?;

    Ok(AudioHandle {
        active_voices,
        is_sustained,
        _stream: stream,
    })
}

pub fn initialize_audio() -> Result<()> {
    println!("[INIT] Loading all FLAC files into RAM...");

    let current_dir =
        std::env::current_dir().map_err(|e| AudioError::InstrumentError(e.to_string()))?;
    println!("[INIT] Current working directory: {:?}", current_dir);

    let samples_dir = current_dir.join("data/instrument/Samples");
    println!("[INIT] Looking for samples at: {:?}", samples_dir);

    if !samples_dir.exists() {
        return Err(AudioError::InstrumentError(format!(
            "Samples directory not found at: {:?}",
            samples_dir
        )));
    }

    let mut total_files = 0;
    let mut loaded_count = 0;

    // Count total FLAC files first
    for entry in
        fs::read_dir(&samples_dir).map_err(|e| AudioError::InstrumentError(e.to_string()))?
    {
        let entry = entry.map_err(|e| AudioError::InstrumentError(e.to_string()))?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("flac") {
            total_files += 1;
        }
    }

    println!("[INIT] Found {} FLAC files to load", total_files);

    // Load all FLAC files into cache
    for entry in
        fs::read_dir(&samples_dir).map_err(|e| AudioError::InstrumentError(e.to_string()))?
    {
        let entry = entry.map_err(|e| AudioError::InstrumentError(e.to_string()))?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("flac") {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            println!(
                "[INIT] [{}/{}] Loading: {} (full path: {})",
                loaded_count + 1,
                total_files,
                file_name,
                path.display()
            );

            let file_path_str = path.to_string_lossy().to_string();
            let decoded = crate::engine::decoder::decode_flac(&file_path_str)
                .map_err(|e| AudioError::FlacDecodeError(file_name.clone(), e.to_string()))?;

            let mut cache = SAMPLE_CACHE.lock().unwrap();
            cache.insert(file_name.clone(), decoded);
            loaded_count += 1;

            println!(
                "[INIT] [{}/{}] {:.1}% complete - {} (cache size: {})",
                loaded_count,
                total_files,
                (loaded_count as f32 / total_files as f32) * 100.0,
                file_name,
                cache.len()
            );
        }
    }

    println!(
        "[INIT] Successfully loaded {} FLAC files into RAM",
        loaded_count
    );
    Ok(())
}
