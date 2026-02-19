import { For, Show } from "solid-js";
import Piano from "./piano";
import { type SectionNum } from "../utils/keyMapping";
import type { Modifier } from "../utils/keyMapping";

interface PianoSectionProps {
  activeNotes: () => Set<number>;
  onNoteOn: (midi: number, hand: 'left' | 'right') => void;
  onNoteOff: (midi: number) => void;
  leftSection: () => SectionNum | null;
  rightSection: () => SectionNum | null;
  leftModifier: () => Modifier;
  rightModifier: () => Modifier;
  leftLayerIdx: () => number;
  rightLayerIdx: () => number;
  availableLayers: () => string[];
}

const LEFT_SECTIONS: { num: SectionNum; key: string; label: string }[] = [
  { num: 1, key: "b", label: "F0–C2" },
  { num: 2, key: "g", label: "F1–C3" },
  { num: 3, key: "t", label: "F2–C4" },
];

const RIGHT_SECTIONS: { num: SectionNum; key: string; label: string }[] = [
  { num: 1, key: "n", label: "F3–C5" },
  { num: 2, key: "h", label: "F4–C6" },
  { num: 3, key: "y", label: "F5–C7" },
];

// Velocity values matching usePiano velocityForLayer logic
const LAYER_VELOCITIES: Record<string, number> = {
  PP: 20, MP: 54, MF: 76, FF: 106,
};

function velocityLabel(layers: string[], idx: number): string {
  const layer = layers[Math.min(idx, layers.length - 1)];
  if (!layer) return "—";
  const total = layers.length;
  const vel = LAYER_VELOCITIES[layer] ??
    Math.round(20 + (idx / Math.max(total - 1, 1)) * 86);
  return `${layer} ${vel}`;
}

function modBadge(mod: Modifier) {
  if (!mod) return null;
  return (
    <span class={`text-[9px] font-black ml-1 ${mod === 'sharp' ? 'text-emerald-400' : 'text-blue-400'}`}>
      {mod === 'sharp' ? '♯' : '♭'}
    </span>
  );
}

export default function PianoSection(props: PianoSectionProps) {
  const { activeNotes, onNoteOn, onNoteOff, leftSection, rightSection } = props;

  const isActive = () => leftSection() !== null || rightSection() !== null;

  return (
    <div class="w-full flex flex-col gap-2 pb-4">

      {/* Status bar */}
      <div class="flex items-center justify-center gap-2 px-4 flex-wrap">

        {/* Left hand sections */}
        <div class="flex items-center gap-1">
          <span class="text-[9px] text-zinc-600 font-bold uppercase tracking-widest mr-0.5">L</span>
          <For each={LEFT_SECTIONS}>{({ num, key, label }) => (
            <div class={`flex items-center gap-1 px-2 py-0.5 rounded border transition-all duration-100 ${leftSection() === num
                ? "bg-blue-600/80 border-blue-400"
                : "bg-zinc-900 border-zinc-800"
              }`}>
              <kbd class={`text-[9px] font-black rounded px-0.5 ${leftSection() === num ? "text-blue-100" : "text-zinc-600"
                }`}>{key.toUpperCase()}</kbd>
              <span class={`text-[9px] font-bold ${leftSection() === num ? "text-white" : "text-zinc-700"
                }`}>{label}</span>
            </div>
          )}</For>
        </div>

        <div class="h-4 w-px bg-zinc-800" />

        {/* Right hand sections */}
        <div class="flex items-center gap-1">
          <For each={RIGHT_SECTIONS}>{({ num, key, label }) => (
            <div class={`flex items-center gap-1 px-2 py-0.5 rounded border transition-all duration-100 ${rightSection() === num
                ? "bg-green-600/80 border-green-400"
                : "bg-zinc-900 border-zinc-800"
              }`}>
              <kbd class={`text-[9px] font-black rounded px-0.5 ${rightSection() === num ? "text-green-100" : "text-zinc-600"
                }`}>{key.toUpperCase()}</kbd>
              <span class={`text-[9px] font-bold ${rightSection() === num ? "text-white" : "text-zinc-700"
                }`}>{label}</span>
            </div>
          )}</For>
          <span class="text-[9px] text-zinc-600 font-bold uppercase tracking-widest ml-0.5">R</span>
        </div>

        <div class="h-4 w-px bg-zinc-800" />

        {/* Per-hand velocity */}
        <Show when={props.availableLayers().length > 0}>
          <div class="flex items-center gap-3 text-[10px]">
            <div class="flex items-center gap-1">
              <span class="text-blue-500 font-black">L</span>
              <span class="text-zinc-300 font-bold">
                {velocityLabel(props.availableLayers(), props.leftLayerIdx())}
              </span>
              {modBadge(props.leftModifier())}
            </div>
            <div class="flex items-center gap-1">
              <span class="text-green-500 font-black">R</span>
              <span class="text-zinc-300 font-bold">
                {velocityLabel(props.availableLayers(), props.rightLayerIdx())}
              </span>
              {modBadge(props.rightModifier())}
            </div>
          </div>

          <div class="h-4 w-px bg-zinc-800" />
        </Show>

        {/* ESC status */}
        <div class="flex items-center gap-1.5">
          <Show
            when={isActive()}
            fallback={
              <span class="text-[9px] text-zinc-700 font-bold italic">
                ESC — no sound · press B/G/T or N/H/Y to activate
              </span>
            }
          >
            <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse inline-block" />
            <span class="text-[9px] text-zinc-500 font-bold">
              active · <kbd class="font-black text-zinc-400">ESC</kbd> to mute
            </span>
          </Show>
        </div>

      </div>

      <Piano
        activeNotes={activeNotes}
        onNoteOn={onNoteOn}
        onNoteOff={onNoteOff}
        leftSection={leftSection}
        rightSection={rightSection}
      />
    </div>
  );
}
