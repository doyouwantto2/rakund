use crate::parser::note_name_to_midi;

/// Compute pitch ratio for a WAV sample.
/// WAV files are typically one-note-per-file (lokey = hikey = pitch = midi),
/// but the same pitch shifting logic applies when they do cover multiple notes.
///
/// recorded_pitch: the pitch field from instrument.json e.g. "C3"
/// target_midi:    the MIDI number the user wants to play
///
/// Returns 1.0 when recorded pitch == target (no shifting needed),
/// otherwise 2^((target - recorded) / 12)
pub fn pitch_ratio(recorded_pitch: &str, target_midi: u8) -> f32 {
    let recorded_midi = note_name_to_midi(recorded_pitch).unwrap_or(target_midi);
    2.0f32.powf((target_midi as f32 - recorded_midi as f32) / 12.0)
}

/// Returns true when this WAV file covers exactly one note (no pitch shifting needed).
pub fn is_exact_note(lokey: u8, hikey: u8, recorded_midi: u8) -> bool {
    lokey == hikey && hikey == recorded_midi
}
