// ── Piano Layout ──────────────────────────────────────────────────────────────
// Single source of truth for all 88-key positions.
// Used by both PianoContainer (rendering keys) and RainLayout (aligning rain keys).
// All `left` and `width` values are percentages (0–100) of the total piano width.

export interface KeyLayout {
  midi: number;
  type: "white" | "black";
  left: number;  // % of total piano width
  width: number; // % of total piano width
}

export const PIANO_KEYS: KeyLayout[] = [
  { midi: 21, type: "white", left: 0.0000, width: 1.9231 },
  { midi: 22, type: "black", left: 1.3462, width: 1.1538 },
  { midi: 23, type: "white", left: 1.9231, width: 1.9231 },
  { midi: 24, type: "white", left: 3.8462, width: 1.9231 },
  { midi: 25, type: "black", left: 5.1923, width: 1.1538 },
  { midi: 26, type: "white", left: 5.7692, width: 1.9231 },
  { midi: 27, type: "black", left: 7.1154, width: 1.1538 },
  { midi: 28, type: "white", left: 7.6923, width: 1.9231 },
  { midi: 29, type: "white", left: 9.6154, width: 1.9231 },
  { midi: 30, type: "black", left: 10.9615, width: 1.1538 },
  { midi: 31, type: "white", left: 11.5385, width: 1.9231 },
  { midi: 32, type: "black", left: 12.8846, width: 1.1538 },
  { midi: 33, type: "white", left: 13.4615, width: 1.9231 },
  { midi: 34, type: "black", left: 14.8077, width: 1.1538 },
  { midi: 35, type: "white", left: 15.3846, width: 1.9231 },
  { midi: 36, type: "white", left: 17.3077, width: 1.9231 },
  { midi: 37, type: "black", left: 18.6538, width: 1.1538 },
  { midi: 38, type: "white", left: 19.2308, width: 1.9231 },
  { midi: 39, type: "black", left: 20.5769, width: 1.1538 },
  { midi: 40, type: "white", left: 21.1538, width: 1.9231 },
  { midi: 41, type: "white", left: 23.0769, width: 1.9231 },
  { midi: 42, type: "black", left: 24.4231, width: 1.1538 },
  { midi: 43, type: "white", left: 25.0000, width: 1.9231 },
  { midi: 44, type: "black", left: 26.3462, width: 1.1538 },
  { midi: 45, type: "white", left: 26.9231, width: 1.9231 },
  { midi: 46, type: "black", left: 28.2692, width: 1.1538 },
  { midi: 47, type: "white", left: 28.8462, width: 1.9231 },
  { midi: 48, type: "white", left: 30.7692, width: 1.9231 },
  { midi: 49, type: "black", left: 32.1154, width: 1.1538 },
  { midi: 50, type: "white", left: 32.6923, width: 1.9231 },
  { midi: 51, type: "black", left: 34.0385, width: 1.1538 },
  { midi: 52, type: "white", left: 34.6154, width: 1.9231 },
  { midi: 53, type: "white", left: 36.5385, width: 1.9231 },
  { midi: 54, type: "black", left: 37.8846, width: 1.1538 },
  { midi: 55, type: "white", left: 38.4615, width: 1.9231 },
  { midi: 56, type: "black", left: 39.8077, width: 1.1538 },
  { midi: 57, type: "white", left: 40.3846, width: 1.9231 },
  { midi: 58, type: "black", left: 41.7308, width: 1.1538 },
  { midi: 59, type: "white", left: 42.3077, width: 1.9231 },
  { midi: 60, type: "white", left: 44.2308, width: 1.9231 },
  { midi: 61, type: "black", left: 45.5769, width: 1.1538 },
  { midi: 62, type: "white", left: 46.1538, width: 1.9231 },
  { midi: 63, type: "black", left: 47.5000, width: 1.1538 },
  { midi: 64, type: "white", left: 48.0769, width: 1.9231 },
  { midi: 65, type: "white", left: 50.0000, width: 1.9231 },
  { midi: 66, type: "black", left: 51.3462, width: 1.1538 },
  { midi: 67, type: "white", left: 51.9231, width: 1.9231 },
  { midi: 68, type: "black", left: 53.2692, width: 1.1538 },
  { midi: 69, type: "white", left: 53.8462, width: 1.9231 },
  { midi: 70, type: "black", left: 55.1923, width: 1.1538 },
  { midi: 71, type: "white", left: 55.7692, width: 1.9231 },
  { midi: 72, type: "white", left: 57.6923, width: 1.9231 },
  { midi: 73, type: "black", left: 59.0385, width: 1.1538 },
  { midi: 74, type: "white", left: 59.6154, width: 1.9231 },
  { midi: 75, type: "black", left: 60.9615, width: 1.1538 },
  { midi: 76, type: "white", left: 61.5385, width: 1.9231 },
  { midi: 77, type: "white", left: 63.4615, width: 1.9231 },
  { midi: 78, type: "black", left: 64.8077, width: 1.1538 },
  { midi: 79, type: "white", left: 65.3846, width: 1.9231 },
  { midi: 80, type: "black", left: 66.7308, width: 1.1538 },
  { midi: 81, type: "white", left: 67.3077, width: 1.9231 },
  { midi: 82, type: "black", left: 68.6538, width: 1.1538 },
  { midi: 83, type: "white", left: 69.2308, width: 1.9231 },
  { midi: 84, type: "white", left: 71.1538, width: 1.9231 },
  { midi: 85, type: "black", left: 72.5000, width: 1.1538 },
  { midi: 86, type: "white", left: 73.0769, width: 1.9231 },
  { midi: 87, type: "black", left: 74.4231, width: 1.1538 },
  { midi: 88, type: "white", left: 75.0000, width: 1.9231 },
  { midi: 89, type: "white", left: 76.9231, width: 1.9231 },
  { midi: 90, type: "black", left: 78.2692, width: 1.1538 },
  { midi: 91, type: "white", left: 78.8462, width: 1.9231 },
  { midi: 92, type: "black", left: 80.1923, width: 1.1538 },
  { midi: 93, type: "white", left: 80.7692, width: 1.9231 },
  { midi: 94, type: "black", left: 82.1154, width: 1.1538 },
  { midi: 95, type: "white", left: 82.6923, width: 1.9231 },
  { midi: 96, type: "white", left: 84.6154, width: 1.9231 },
  { midi: 97, type: "black", left: 85.9615, width: 1.1538 },
  { midi: 98, type: "white", left: 86.5385, width: 1.9231 },
  { midi: 99, type: "black", left: 87.8846, width: 1.1538 },
  { midi: 100, type: "white", left: 88.4615, width: 1.9231 },
  { midi: 101, type: "white", left: 90.3846, width: 1.9231 },
  { midi: 102, type: "black", left: 91.7308, width: 1.1538 },
  { midi: 103, type: "white", left: 92.3077, width: 1.9231 },
  { midi: 104, type: "black", left: 93.6538, width: 1.1538 },
  { midi: 105, type: "white", left: 94.2308, width: 1.9231 },
  { midi: 106, type: "black", left: 95.5769, width: 1.1538 },
  { midi: 107, type: "white", left: 96.1538, width: 1.9231 },
  { midi: 108, type: "white", left: 98.0769, width: 1.9231 },
];

// ── Lookup ────────────────────────────────────────────────────────────────────

const KEY_MAP = new Map<number, KeyLayout>(PIANO_KEYS.map(k => [k.midi, k]));

export function getKeyLayout(midi: number): KeyLayout | undefined {
  return KEY_MAP.get(midi);
}

// ── Pixel conversion ──────────────────────────────────────────────────────────

export interface KeyLayoutPx {
  x: number;
  width: number;
  type: "white" | "black";
}

export function getKeyLayoutPx(
  midi: number,
  totalWidthPx: number,
): KeyLayoutPx | undefined {
  const key = KEY_MAP.get(midi);
  if (!key) return undefined;
  return {
    x: (key.left / 100) * totalWidthPx,
    width: (key.width / 100) * totalWidthPx,
    type: key.type,
  };
}

// ── Octave boundaries ─────────────────────────────────────────────────────────
// The right edge of each B note = the left edge of the next C.
// These are the vertical divider lines shown in the rain scene.
// B notes in the 88-key range: B0–B7

export const OCTAVE_BOUNDARY_MIDIS: number[] = [23, 35, 47, 59, 71, 83, 95, 107];

// Returns right-edge percentages for each octave boundary
export function getOctaveBoundariesPct(): number[] {
  return OCTAVE_BOUNDARY_MIDIS.map(midi => {
    const key = KEY_MAP.get(midi);
    if (!key) return 0;
    return key.left + key.width;
  });
}

// Returns right-edge pixel positions for each octave boundary
export function getOctaveBoundariesPx(totalWidthPx: number): number[] {
  return getOctaveBoundariesPct().map(pct => (pct / 100) * totalWidthPx);
}
