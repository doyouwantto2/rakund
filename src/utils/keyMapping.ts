/**
 * Piano keyboard layout
 *
 * LEFT HAND (12 keys):  z x c v a s d f q w e r   (z = lowest)
 * RIGHT HAND (12 keys): m , . / j k l ; u i o p   (m = lowest)
 *
 * SECTION SELECTORS — switch (not toggle) the active section:
 *   Left:  b=1  g=2  t=3
 *   Right: n=1  h=2  y=3
 *
 * Each section maps the 12 keys to 12 consecutive white keys on the piano:
 *
 *   Left b(1): z=A0 x=B0 c=C1 v=D1 a=E1 s=F1 d=G1 f=A1 q=B1 w=C2 e=D2 r=E2
 *   Left g(2): z=A1 x=B1 c=C2 v=D2 a=E2 s=F2 d=G2 f=A2 q=B2 w=C3 e=D3 r=E3
 *   Left t(3): z=A2 x=B2 c=C3 v=D3 a=E3 s=F3 d=G3 f=A3 q=B3 w=C4 e=D4 r=E4
 *
 *   Right n(1): m=F4 ,=G4 .=A4 /=B4 j=C5 k=D5 l=E5 ;=F5 u=G5 i=A5 o=B5 p=C6
 *   Right h(2): m=F5 ,=G5 .=A5 /=B5 j=C6 k=D6 l=E6 ;=F6 u=G6 i=A6 o=B6 p=C7
 *   Right y(3): m=F6 ,=G6 .=A6 /=B6 j=C7 k=D7 l=E7 ;=F7 u=G7 i=A7 o=B7 p=C8
 *
 * MODIFIERS (black keys):
 *   Shift (+1): sharp — only fires if result is a black key
 *   Alt   (-1): flat  — only fires if result is a black key
 *   Both Shifts (+2) / Both Alts (-2): two semitones
 *   Blocked cases: Shift+E, Shift+B, Alt+F, Alt+C (would land on white)
 *
 * ESC clears both sections.
 */

export const LEFT_KEY_ORDER = ['z', 'x', 'c', 'v', 'a', 's', 'd', 'f', 'q', 'w', 'e', 'r'];
export const RIGHT_KEY_ORDER = ['m', ',', '.', '/', 'j', 'k', 'l', ';', 'u', 'i', 'o', 'p'];

const LEFT_KEYS = new Set(LEFT_KEY_ORDER);
const RIGHT_KEYS = new Set(RIGHT_KEY_ORDER);

// Section selectors — pressing switches to that section (replaces current)
export const LEFT_SECTION_KEYS = { 'b': 1, 'g': 2, 't': 3 } as const;
export const RIGHT_SECTION_KEYS = { 'n': 1, 'h': 2, 'y': 3 } as const;

export type SectionNum = 1 | 2 | 3;

// 12 white-key MIDI values per section (precomputed from piano white key positions)
export const SECTION_WHITES: Record<'left' | 'right', Record<SectionNum, number[]>> = {
  left: {
    1: [21, 23, 24, 26, 28, 29, 31, 33, 35, 36, 38, 40], // A0–E2
    2: [33, 35, 36, 38, 40, 41, 43, 45, 47, 48, 50, 52], // A1–E3
    3: [45, 47, 48, 50, 52, 53, 55, 57, 59, 60, 62, 64], // A2–E4
  },
  right: {
    1: [65, 67, 69, 71, 72, 74, 76, 77, 79, 81, 83, 84], // F4–C6
    2: [77, 79, 81, 83, 84, 86, 88, 89, 91, 93, 95, 96], // F5–C7
    3: [89, 91, 93, 95, 96, 98, 100, 101, 103, 105, 107, 108], // F6–C8
  },
};

const BLACK_SEMITONES = new Set([1, 3, 6, 8, 10]);

function applyOffset(whiteMidi: number, offset: number): number | null {
  if (offset === 0) return whiteMidi;
  const result = whiteMidi + offset;
  if (result < 21 || result > 108) return null;
  if (!BLACK_SEMITONES.has(result % 12)) return null; // would land on white — block
  return result;
}

/**
 * Main lookup — returns MIDI note(s) for a key press.
 * activeSection is a single section (null = hand not active).
 */
export function getMidiForKey(
  key: string,
  activeSection: SectionNum | null,
  hand: 'left' | 'right',
  semitoneOffset: number = 0,
): number[] {
  if (activeSection === null) return [];
  const keyOrder = hand === 'left' ? LEFT_KEY_ORDER : RIGHT_KEY_ORDER;
  const validKeys = hand === 'left' ? LEFT_KEYS : RIGHT_KEYS;
  if (!validKeys.has(key)) return [];

  const keyIndex = keyOrder.indexOf(key);
  if (keyIndex === -1) return [];

  const whiteMidi = SECTION_WHITES[hand][activeSection][keyIndex];
  if (whiteMidi === undefined) return [];

  const midi = applyOffset(whiteMidi, semitoneOffset);
  return midi !== null ? [midi] : [];
}

/**
 * All MIDI notes (chromatic) in a section — for piano highlighting.
 */
export function getSectionMidiRange(
  hand: 'left' | 'right',
  section: SectionNum,
): number[] {
  const whites = SECTION_WHITES[hand][section];
  const notes: number[] = [];
  for (let m = whites[0]; m <= whites[whites.length - 1]; m++) {
    notes.push(m);
  }
  return notes;
}

export function getKeyHighlight(
  midi: number,
  leftSection: SectionNum | null,
  rightSection: SectionNum | null,
): 'left' | 'right' | null {
  const inLeft = leftSection !== null && getSectionMidiRange('left', leftSection).includes(midi);
  const inRight = rightSection !== null && getSectionMidiRange('right', rightSection).includes(midi);
  if (inLeft && inRight) return 'left'; // Prioritize left when both sections overlap
  if (inLeft) return 'left';
  if (inRight) return 'right';
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
