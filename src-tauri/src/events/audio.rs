use crate::engine::{decoder, player};
use crate::setup::sound::AudioHandle;
use tauri::{command, State};

#[command]
pub async fn play_note(path: String, handle: State<'_, AudioHandle>) -> Result<(), String> {
    let sound_data = decoder::decode_flac(&path);

    player::play_sound(&handle.active_voices, sound_data);

    Ok(())
}
