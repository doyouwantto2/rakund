use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct Voice {
    pub data: Arc<Vec<f32>>,
    pub playhead: usize,
}

pub struct AudioHandle {
    pub active_voices: Arc<Mutex<Vec<Voice>>>,
    pub _stream: cpal::Stream,
}

pub fn start_stream() -> AudioHandle {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device");
    let config = device.default_output_config().expect("No config");

    let active_voices = Arc::new(Mutex::new(Vec::<Voice>::new()));
    let voices_clone = Arc::clone(&active_voices);

    let stream = device
        .build_output_stream(
            &config.into(),
            move |output: &mut [f32], _| {
                if let Ok(mut voices) = voices_clone.lock() {
                    for frame in output.iter_mut() {
                        let mut mixed = 0.0;
                        voices.retain_mut(|v| {
                            if v.playhead < v.data.len() {
                                mixed += v.data[v.playhead];
                                v.playhead += 1;
                                true
                            } else {
                                false
                            }
                        });
                        *frame = mixed.clamp(-1.0, 1.0);
                    }
                }
            },
            |err| eprintln!("Audio error: {err}"),
            None,
        )
        .unwrap();

    stream.play().unwrap();

    AudioHandle {
        active_voices,
        _stream: stream,
    }
}
