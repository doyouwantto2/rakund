use crate::setup::models::SplendidConfig;
use crate::setup::sound;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn play_midi_note(
    midi_num: u8,
    velocity: u8,
    handle: State<'_, sound::AudioHandle>,
    config: State<'_, SplendidConfig>,
    _app: AppHandle,
) -> Result<(), String> {
    let key_data = config
        .keys
        .get(&midi_num.to_string())
        .ok_or_else(|| format!("Note {} not found in piano configuration", midi_num))?;

    let sample_info = key_data
        .samples
        .iter()
        .find(|s| velocity >= s.min_vel && velocity <= s.max_vel)
        .or_else(|| key_data.samples.first())
        .ok_or_else(|| format!("No samples found for note {}", midi_num))?;

    let pitch_ratio = 2.0f32.powf((midi_num as f32 - key_data.midi_note as f32) / 12.0);

    // Use pre-decoded cache instead of decoding from disk
    let data = config
        .samples_cache
        .get(&sample_info.file)
        .ok_or_else(|| format!("Sample {} not found in cache", sample_info.file))?
        .clone();

    if let Ok(mut voices_guard) = handle.active_voices.lock() {
        let voices: &mut Vec<sound::Voice> = &mut voices_guard;
        voices.push(sound::Voice {
            data,
            playhead: 0.0,
            pitch_ratio,
            midi_note: midi_num,
            is_releasing: false,
            volume: (velocity as f32 / 127.0),
        });
    }

    Ok(())
}
