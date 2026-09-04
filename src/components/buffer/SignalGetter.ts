import { createSignal } from "solid-js";

export interface PianoEvent {
  midi: number;
  velocity: number;
  action: "on" | "off";
  timestamp: number;
  source: "user";
}

const [pianoEvent, setPianoEvent] = createSignal<PianoEvent | null>(null);

export { pianoEvent, setPianoEvent };
