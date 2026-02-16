use crate::setup::audio::Voice;
use std::sync::{Arc, Mutex};

pub fn play_sound(active_voices: &Arc<Mutex<Vec<Voice>>>, data: Arc<Vec<f32>>) {
    if let Ok(mut voices) = active_voices.lock() {
        voices.push(Voice { data, playhead: 0 });
    }
}
