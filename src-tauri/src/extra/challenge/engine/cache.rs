#[derive(Debug, Clone)]
pub struct MidiNote {
    pub note: u8,
    pub velocity: u8,
    pub start_time: u32,
    pub duration: u32,
    pub channel: u8,
}

#[derive(Debug, Clone)]
pub struct MidiTrack {
    pub notes: Vec<MidiNote>,
    pub name: Option<String>,
    pub instrument: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MidiFile {
    pub tracks: Vec<MidiTrack>,
    pub format: u16,
    pub division: u16,
    pub tempo: u32,
}

impl MidiFile {
    pub fn get_all_notes(&self) -> Vec<MidiNote> {
        let mut all_notes = Vec::new();
        for track in &self.tracks {
            all_notes.extend(track.notes.clone());
        }
        all_notes.sort_by_key(|note| note.start_time);
        all_notes
    }

    pub fn get_track_notes(&self, track_index: usize) -> Option<&[MidiNote]> {
        self.tracks
            .get(track_index)
            .map(|track| track.notes.as_slice())
    }

    pub fn get_total_duration(&self) -> u32 {
        self.tracks
            .iter()
            .map(|track| {
                track
                    .notes
                    .iter()
                    .map(|note| note.start_time + note.duration)
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
    }

    pub fn get_tempo_bpm(&self) -> f32 {
        self.tempo as f32 / 4.0
    }
}