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
    let device = host.default_output_device().expect("No output device");
    let config = device.default_output_config().expect("No config");

    let active_voices = Arc::new(Mutex::new(Vec::<Voice>::new()));
    let is_sustained = Arc::new(Mutex::new(false));

    let voices_clone = Arc::clone(&active_voices);
    let sustain_clone = Arc::clone(&is_sustained);

    let stream = device
        .build_output_stream(
            &config.into(),
            move |output: &mut [f32], _| {
                // Copy voices data to minimize lock time
                let (mut voices_snapshot, pedal_active) = {
                    let pedal = *sustain_clone.lock().unwrap();
                    let voices = voices_clone.lock().unwrap();
                    (voices.clone(), pedal)
                };

                // Pre-calculate gain reduction
                let num_voices: f32 = voices_snapshot.len() as f32;
                let gain_reduction = if num_voices > 1.0 {
                    1.0 / (num_voices.sqrt())
                } else {
                    1.0
                };

                // Process all frames without holding locks
                for frame in output.iter_mut() {
                    let mut mixed: f32 = 0.0;
                    
                    for v in &mut voices_snapshot {
                        let pos = v.playhead as usize;
                        if pos < v.data.len() && v.volume > 0.001 {
                            mixed += v.data[pos] * v.volume;
                            v.playhead += v.pitch_ratio;

                            if v.is_releasing && !pedal_active {
                                v.volume *= 0.97;
                            }
                        }
                    }

                    let final_sample: f32 = mixed * gain_reduction * 0.8;
                    *frame = final_sample.clamp(-1.0, 1.0);
                }

                // Update the actual voices with modified data
                if let Ok(mut voices) = voices_clone.lock() {
                    *voices = voices_snapshot;
                }
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
