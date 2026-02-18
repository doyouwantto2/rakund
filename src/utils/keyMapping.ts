/**
 * Key to MIDI mapping for piano — two octave layout, no conflicts
 *
 * Lower octave (C3–B3): z x c v b n m  (white keys, bottom row)
 *                        s d   g h j    (black keys, home row)
 *
 * Upper octave (C4–B4): a w e r t y u  (white + black interleaved)
 *   White: a f ; k l
 *   Black: w e t y u
 *
 * Extended: q o p i for upper whites/blacks
 */

export const keyToMidi: Record<string, number> = {
  // --- Lower octave: C3 to B3 ---
  // White keys
  'z': 48, // C3
  'x': 50, // D3
  'c': 52, // E3
  'v': 53, // F3
  'b': 55, // G3
  'n': 57, // A3
  'm': 59, // B3
  // Black keys
  's': 49, // C#3
  'd': 51, // D#3
  'g': 54, // F#3
  'h': 56, // G#3
  'j': 58, // A#3

  // --- Upper octave: C4 to B4 ---
  // White keys
  'a': 60, // C4 (middle C)
  'f': 62, // D4
  ';': 64, // E4
  'k': 65, // F4
  'l': 67, // G4
  'q': 69, // A4
  'i': 71, // B4
  // Black keys
  'w': 61, // C#4
  'e': 63, // D#4
  't': 66, // F#4
  'y': 68, // G#4
  'u': 70, // A#4

  // --- Extra high: C5–E5 ---
  'o': 72, // C5
  'p': 74, // D5
  'r': 76, // E5
};

/**
 * Get MIDI note from keyboard key
 */
export function getKeyToMidi(key: string): number | undefined {
  return keyToMidi[key.toLowerCase()];
}

/**
 * Add or update key mapping
 */
export function addKeyMapping(key: string, midi: number): void {
  keyToMidi[key.toLowerCase()] = midi;
}
