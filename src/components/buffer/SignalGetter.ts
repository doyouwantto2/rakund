import { createSignal } from "solid-js";

// ── Types ─────────────────────────────────────────────────────────────────────

export interface PianoEvent {
  midi: number;
  velocity: number;
  action: "on" | "off";
  timestamp: number; // performance.now() — used for scoring timing
  source: "user";
}

// ── Channel ───────────────────────────────────────────────────────────────────
// Piano WRITES, Buffer READS.
// This is a one-way channel: user input flows from PianoContainer → BufferContainer.
// Buffer watches this signal to calculate score in perform mode.
// Nothing related to instruction mode should ever write to this signal.

const [pianoEvent, setPianoEvent] = createSignal<PianoEvent | null>(null);

export { pianoEvent, setPianoEvent };
