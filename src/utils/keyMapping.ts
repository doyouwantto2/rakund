/**
 * Simple key to MIDI mapping for piano
 */

// Default keyboard mapping
export const keyToMidi: Record<string, number> = {
  'a': 60, 'w': 61, 's': 62, 'e': 63, 'd': 64, 'f': 65, 't': 66,
  'g': 67, 'y': 68, 'h': 69, 'u': 70, 'j': 71, 'k': 72, 'o': 73, 'l': 74
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
