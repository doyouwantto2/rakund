use crate::parser::note_name_to_midi;

pub fn pitch_ratio(recorded_pitch: &str, target_midi: u8) -> f32 {
    let recorded_midi = note_name_to_midi(recorded_pitch).unwrap_or(target_midi);
    2.0f32.powf((target_midi as f32 - recorded_midi as f32) / 12.0)
}
