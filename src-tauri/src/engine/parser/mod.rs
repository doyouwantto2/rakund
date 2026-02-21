pub mod flac;
pub mod wav;

pub fn note_name_to_midi(name: &str) -> Option<u8> {
    let name = name.trim();
    if name.is_empty() {
        return None;
    }

    let mut chars = name.chars().peekable();

    let semitone = match chars.next()?.to_ascii_uppercase() {
        'C' => 0i32,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return None,
    };

    let accidental = match chars.peek() {
        Some('#') => {
            chars.next();
            1i32
        }
        Some('b') => {
            chars.next();
            -1i32
        }
        _ => 0i32,
    };

    let octave: i32 = chars.collect::<String>().parse().ok()?;

    let midi = (octave + 1) * 12 + semitone + accidental;
    if !(0..=127).contains(&midi) {
        return None;
    }

    Some(midi as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_conversion() {
        assert_eq!(note_name_to_midi("C-1"), Some(0));
        assert_eq!(note_name_to_midi("A0"), Some(21));
        assert_eq!(note_name_to_midi("B-1"), Some(23));
        assert_eq!(note_name_to_midi("C1"), Some(24));
        assert_eq!(note_name_to_midi("C4"), Some(60));
        assert_eq!(note_name_to_midi("C#4"), Some(61));
        assert_eq!(note_name_to_midi("A4"), Some(69));
        assert_eq!(note_name_to_midi("C8"), Some(108));
    }
}
