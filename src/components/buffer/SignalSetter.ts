import { createSignal } from "solid-js";

export interface PianoCommand {
  midi: number;
  velocity: number;
  action: "on" | "off";
  source: "instruction";
}

const [pianoCommand, setPianoCommand] = createSignal<PianoCommand | null>(null);

export { pianoCommand, setPianoCommand };
