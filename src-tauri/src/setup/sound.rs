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

pub fn start_stream() -> AudioHandle {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");
    let config = device
        .default_output_config()
        .expect("Failed to get config");

    let active_voices = Arc::new(Mutex::new(Vec::<Voice>::new()));
    let is_sustained = Arc::new(Mutex::new(false));

    let voices_clone = Arc::clone(&active_voices);

    let stream = device
        .build_output_stream(
            &config.into(),
            move |output: &mut [f32], _| {
                let mut voices = match voices_clone.try_lock() {
                    Ok(guard) => guard,
                    Err(_) => return,
                };

                let num_voices = voices.len() as f32;
                let gain_reduction = if num_voices > 1.0 {
                    1.0 / (num_voices.sqrt() * 1.2)
                } else {
                    1.0
                };

                for frame in output.iter_mut() {
                    let mut mixed: f32 = 0.0;

                    for v in voices.iter_mut() {
                        let pos = v.playhead as usize;

                        if pos < v.data.len() && v.volume > 0.0005 {
                            mixed += v.data[pos] * v.volume;

                            v.playhead += v.pitch_ratio;

                            if v.is_releasing {
                                v.volume *= 0.999999;
                            }
                        }
                    }

                    let final_sample = mixed * gain_reduction * 0.7;
                    *frame = final_sample.clamp(-1.0, 1.0);
                }

                voices.retain(|v| (v.playhead as usize) < v.data.len() && v.volume > 0.0005);
            },
            |err| eprintln!("Audio error: {err}"),
            None,
        )
        .unwrap();

    stream.play().unwrap();

    AudioHandle {
        active_voices,
        is_sustained,
        _stream: stream,
    }
}
