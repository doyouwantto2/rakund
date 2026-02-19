export const LEFT_KEY_ORDER = ['z', 'x', 'c', 'v', 'a', 's', 'd', 'f', 'q', 'w', 'e', 'r'];
export const RIGHT_KEY_ORDER = ['m', ',', '.', '/', 'j', 'k', 'l', ';', 'u', 'i', 'o', 'p'];

const LEFT_KEYS = new Set(LEFT_KEY_ORDER);
const RIGHT_KEYS = new Set(RIGHT_KEY_ORDER);

export const LEFT_SECTION_KEYS = { 'b': 1, 'g': 2, 't': 3 } as const;
export const RIGHT_SECTION_KEYS = { 'n': 1, 'h': 2, 'y': 3 } as const;

export type SectionNum = 1 | 2 | 3;
export type Modifier = 'sharp' | 'flat' | null;

export const SECTION_WHITES: Record<'left' | 'right', Record<SectionNum, number[]>> = {
  left: {
    1: [21, 23, 24, 26, 28, 29, 31, 33, 35, 36, 38, 40],
    2: [33, 35, 36, 38, 40, 41, 43, 45, 47, 48, 50, 52],
    3: [45, 47, 48, 50, 52, 53, 55, 57, 59, 60, 62, 64],
  },
  right: {
    1: [65, 67, 69, 71, 72, 74, 76, 77, 79, 81, 83, 84],
    2: [77, 79, 81, 83, 84, 86, 88, 89, 91, 93, 95, 96],
    3: [89, 91, 93, 95, 96, 98, 100, 101, 103, 105, 107, 108],
  },
};

function resolveNote(whiteMidi: number, modifier: Modifier): number | null {
  if (modifier === null) return whiteMidi;

  // Sharp strictly adds 1 semitone, flat strictly subtracts 1 semitone
  const result = modifier === 'sharp' ? whiteMidi + 1 : whiteMidi - 1;

  // Keep it within standard 88-key piano range (A0=21 to C8=108)
  if (result < 21 || result > 108) return null;

  return result;
}

export function getMidiForKey(
  key: string,
  activeSection: SectionNum | null,
  hand: 'left' | 'right',
  modifier: Modifier = null,
): number | null {
  if (activeSection === null) return null;
  const keyOrder = hand === 'left' ? LEFT_KEY_ORDER : RIGHT_KEY_ORDER;
  const validKeys = hand === 'left' ? LEFT_KEYS : RIGHT_KEYS;

  if (!validKeys.has(key)) return null;
  const idx = keyOrder.indexOf(key);
  if (idx === -1) return null;

  const whiteMidi = SECTION_WHITES[hand][activeSection][idx];
  return resolveNote(whiteMidi, modifier);
}

function getSectionRange(hand: 'left' | 'right', section: SectionNum): number[] {
  const whites = SECTION_WHITES[hand][section];
  const notes: number[] = [];
  for (let m = whites[0]; m <= whites[whites.length - 1]; m++) notes.push(m);
  return notes;
}

export function getKeyHighlight(
  midi: number,
  leftSection: SectionNum | null,
  rightSection: SectionNum | null,
): 'left' | 'right' | null {
  if (leftSection !== null && getSectionRange('left', leftSection).includes(midi)) return 'left';
  if (rightSection !== null && getSectionRange('right', rightSection).includes(midi)) return 'right';
  return null;
}

export function isLeftSectionKey(key: string): key is keyof typeof LEFT_SECTION_KEYS {
  return key in LEFT_SECTION_KEYS;
}

export function isRightSectionKey(key: string): key is keyof typeof RIGHT_SECTION_KEYS {
  return key in RIGHT_SECTION_KEYS;
}

export function isPianoKey(key: string): boolean {
  return LEFT_KEYS.has(key) || RIGHT_KEYS.has(key);
}
