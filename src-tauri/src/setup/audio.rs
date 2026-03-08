use crate::error::{AudioError, Result};
use crate::extra::sketch::instrument::release;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

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

                // Remove voices that have finished their release envelope
                voices.retain(|v| v.volume > 0.001); // Remove when volume is essentially silent

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
