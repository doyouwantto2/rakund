import { createSignal } from "solid-js";

// ── Types ─────────────────────────────────────────────────────────────────────

export interface PianoCommand {
  midi: number;
  velocity: number;
  action: "on" | "off";
  source: "instruction"; // tag ensures this is never confused with user input
}

// ── Channel ───────────────────────────────────────────────────────────────────
// Buffer WRITES, Piano READS.
// Reserved for instruction mode ONLY — not wired into anything yet.
// DO NOT read this signal in PianoContainer until instruction mode is designed.
// DO NOT write to this signal from perform mode logic.

const [pianoCommand, setPianoCommand] = createSignal<PianoCommand | null>(null);

export { pianoCommand, setPianoCommand };
