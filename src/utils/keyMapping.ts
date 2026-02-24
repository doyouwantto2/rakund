// ── Key Orders ────────────────────────────────────────────────────────────────
// 12 keys per hand.
// Left:  z x c v a s d f q w e r  ('a' naturally maps to C in the sequence)
// Right: m , . / j k l ; u i o p  ('j' naturally maps to C in the sequence)
export const LEFT_KEY_ORDER = [
  "z",
  "x",
  "c",
  "v",
  "a",
  "s",
  "d",
  "f",
  "q",
  "w",
  "e",
  "r",
] as const;

export const RIGHT_KEY_ORDER = [
  "m",
  ",",
  ".",
  "/",
  "j",
  "k",
  "l",
  ";",
  "u",
  "i",
  "o",
  "p",
] as const;

const LEFT_KEY_SET = new Set<string>(LEFT_KEY_ORDER);
const RIGHT_KEY_SET = new Set<string>(RIGHT_KEY_ORDER);

// ── Octave navigation keys ────────────────────────────────────────────────────
// g → left  hand next octave
// b → left  hand previous octave
// h → right hand next octave
// n → right hand previous octave
export const LEFT_OCTAVE_UP_KEY = "g";
export const LEFT_OCTAVE_DOWN_KEY = "b";
export const RIGHT_OCTAVE_UP_KEY = "h";
export const RIGHT_OCTAVE_DOWN_KEY = "n";

// ── Base white MIDI notes ─────────────────────────────────────────────────────
// Left hand base (octaveOffset = 0) — starts at F1 so 'a' (index 4) = C2:
//   z    x    c    v    a    s    d    f    q    w    e    r
//   F1   G1   A1   B1   C2   D2   E2   F2   G2   A2   B2   C3
//   29   31   33   35   36   38   40   41   43   45   47   48
const LEFT_BASE_WHITES: number[] = [
  29, 31, 33, 35, 36, 38, 40, 41, 43, 45, 47, 48,
];

// Right hand base (octaveOffset = 0) — starts at F4 so 'j' (index 4) = C5:
//   m    ,    .    /    j    k    l    ;    u    i    o    p
//   F4   G4   A4   B4   C5   D5   E5   F5   G5   A5   B5   C6
//   65   67   69   71   72   74   76   77   79   81   83   84
const RIGHT_BASE_WHITES: number[] = [
  65, 67, 69, 71, 72, 74, 76, 77, 79, 81, 83, 84,
];

// ── Piano range ───────────────────────────────────────────────────────────────
const MIDI_MIN = 21; // A0
const MIDI_MAX = 108; // C8

// ── Octave offset clamp limits ────────────────────────────────────────────────
// Valid = all 12 notes in range. We allow 1 extra "fake" octave on each side.
//   Left  valid [0..5]  → clamp [-1..6]
//     offset 0: F1(29)–C3(48)   offset 5: F6(89)–C8(108)
//   Right valid [-3..2] → clamp [-4..3]
//     offset -3: F1(29)–C3(48)  offset 2: F6(89)–C8(108)
export const LEFT_OCTAVE_MIN = -1;
export const LEFT_OCTAVE_MAX = 6;
export const RIGHT_OCTAVE_MIN = -4;
export const RIGHT_OCTAVE_MAX = 3;

// ── Modifier ──────────────────────────────────────────────────────────────────
export type Modifier = "sharp" | "flat" | null;

function applyModifier(midi: number, modifier: Modifier): number | null {
  if (modifier === null) return midi;
  const result = modifier === "sharp" ? midi + 1 : midi - 1;
  if (result < MIDI_MIN || result > MIDI_MAX) return null;
  return result;
}

// ── Public API ────────────────────────────────────────────────────────────────

/**
 * Returns all 12 MIDI notes for a hand at a given octave offset.
 * Notes that fall outside the 88-key range are returned as null (silent).
 * The octave offset itself is never clamped here — callers are responsible
 * for clamping to [LEFT/RIGHT]_OCTAVE_[MIN/MAX].
 */
export function getNotesForHand(
  hand: "left" | "right",
  octaveOffset: number,
): (number | null)[] {
  const base = hand === "left" ? LEFT_BASE_WHITES : RIGHT_BASE_WHITES;
  return base.map((n) => {
    const midi = n + octaveOffset * 12;
    return midi >= MIDI_MIN && midi <= MIDI_MAX ? midi : null;
  });
}

/**
 * Returns the MIDI note for a specific key press, given the current octave offset.
 * Returns null if:
 *   - the key doesn't belong to this hand, or
 *   - the resulting note is outside the 88-key piano range.
 */
export function getMidiForKey(
  key: string,
  hand: "left" | "right",
  octaveOffset: number,
  modifier: Modifier = null,
): number | null {
  const keyOrder = hand === "left" ? LEFT_KEY_ORDER : RIGHT_KEY_ORDER;
  const keySet = hand === "left" ? LEFT_KEY_SET : RIGHT_KEY_SET;

  if (!keySet.has(key)) return null;
  const idx = (keyOrder as readonly string[]).indexOf(key);
  if (idx === -1) return null;

  const base = hand === "left" ? LEFT_BASE_WHITES : RIGHT_BASE_WHITES;
  const rawMidi = base[idx] + octaveOffset * 12;
  if (rawMidi < MIDI_MIN || rawMidi > MIDI_MAX) return null;
  return applyModifier(rawMidi, modifier);
}

/**
 * Returns which hand (if any) highlights a given MIDI note, based on the
 * current octave offsets. Only in-range notes are considered.
 */
export function getKeyHighlight(
  midi: number,
  leftOctave: number,
  rightOctave: number,
): "left" | "right" | "both" | null {
  const inLeft = getRangeForHand("left", leftOctave).has(midi);
  const inRight = getRangeForHand("right", rightOctave).has(midi);
  if (inLeft && inRight) return "both";
  if (inLeft) return "left";
  if (inRight) return "right";
  return null;
}

// ── Range helper ──────────────────────────────────────────────────────────────
// Highlighted range = every MIDI from the lowest to highest in-range note the
// hand can play (whites + blacks in between), at the current octave offset.
// Out-of-range notes are excluded, so the range shrinks at the extremes.
function getRangeForHand(
  hand: "left" | "right",
  octaveOffset: number,
): Set<number> {
  const notes = getNotesForHand(hand, octaveOffset).filter(
    (n): n is number => n !== null,
  );
  if (notes.length === 0) return new Set();
  const lo = Math.min(...notes);
  const hi = Math.max(...notes);
  const set = new Set<number>();
  for (let m = lo; m <= hi; m++) set.add(m);
  return set;
}

// ── Key classification helpers ────────────────────────────────────────────────

export function isLeftPianoKey(key: string): boolean {
  return LEFT_KEY_SET.has(key);
}
export function isRightPianoKey(key: string): boolean {
  return RIGHT_KEY_SET.has(key);
}
export function isPianoKey(key: string): boolean {
  return LEFT_KEY_SET.has(key) || RIGHT_KEY_SET.has(key);
}

export function isLeftOctaveUpKey(key: string): boolean {
  return key === LEFT_OCTAVE_UP_KEY;
}
export function isLeftOctaveDownKey(key: string): boolean {
  return key === LEFT_OCTAVE_DOWN_KEY;
}
export function isRightOctaveUpKey(key: string): boolean {
  return key === RIGHT_OCTAVE_UP_KEY;
}
export function isRightOctaveDownKey(key: string): boolean {
  return key === RIGHT_OCTAVE_DOWN_KEY;
}
