import { Show } from "solid-js";
import type { Modifier } from "../../utils/keyMapping";

interface OctaveControlsProps {
  leftOctave: () => number;
  rightOctave: () => number;
  leftModifier: () => Modifier;
  rightModifier: () => Modifier;
  leftLayerIdx: () => number;
  rightLayerIdx: () => number;
  availableLayers: () => string[];
  velocityForLayer: (layer: string) => number;
  onLeftOctaveChange: (delta: 1 | -1) => void;
  onRightOctaveChange: (delta: 1 | -1) => void;
}

function modBadge(mod: Modifier) {
  if (!mod) return null;
  return (
    <span
      class={`text-[8px] font-black ml-1 ${mod === "sharp" ? "text-emerald-400" : "text-blue-400"}`}
    >
      {mod === "sharp" ? "♯" : "♭"}
    </span>
  );
}

export default function OctaveControls(props: OctaveControlsProps) {
  const leftVelLabel = () => {
    const layers = props.availableLayers();
    if (layers.length === 0) return null;
    const layer = layers[Math.min(props.leftLayerIdx(), layers.length - 1)];
    return `${layer} ${props.velocityForLayer(layer)}`;
  };

  const rightVelLabel = () => {
    const layers = props.availableLayers();
    if (layers.length === 0) return null;
    const layer = layers[Math.min(props.rightLayerIdx(), layers.length - 1)];
    return `${layer} ${props.velocityForLayer(layer)}`;
  };

  const octaveLabel = (o: number) => (o >= 0 ? `+${o}` : `${o}`);

  return (
    <div class="flex items-center gap-3 text-[10px]">
      {/* ── Left hand ─────────────────────────────────────────── */}
      <div class="flex items-center gap-1.5">
        <span class="text-[8px] text-zinc-600 font-bold uppercase tracking-widest w-3">
          L
        </span>

        {/* Previous octave (B) */}
        <button
          onClick={() => props.onLeftOctaveChange(-1)}
          class="flex items-center justify-center w-6 h-6 rounded border border-zinc-700 bg-zinc-900 text-zinc-400 hover:border-zinc-500 hover:text-zinc-200 text-[9px] font-bold transition-colors select-none"
          title="Left hand: previous octave (B)"
        >
          B
        </button>

        {/* Octave badge */}
        <div class="flex items-center justify-center w-8 h-6 rounded border border-blue-600 bg-blue-950 select-none">
          <span class="text-[0.8rem] font-black text-blue-300 tabular-nums leading-none">
            {octaveLabel(props.leftOctave())}
          </span>
        </div>

        {/* Next octave (G) */}
        <button
          onClick={() => props.onLeftOctaveChange(1)}
          class="flex items-center justify-center w-6 h-6 rounded border border-zinc-700 bg-zinc-900 text-zinc-400 hover:border-zinc-500 hover:text-zinc-200 text-[9px] font-bold transition-colors select-none"
          title="Left hand: next octave (G)"
        >
          G
        </button>

        {/* Velocity */}
        <Show when={leftVelLabel() !== null}>
          <div class="flex items-center gap-0.5">
            <span class="text-blue-400 font-black text-[9px]">L</span>
            <span class="text-zinc-300 font-bold">{leftVelLabel()}</span>
            {modBadge(props.leftModifier())}
          </div>
        </Show>
      </div>

      <div class="h-4 w-px bg-zinc-800" />

      {/* ── Right hand ────────────────────────────────────────── */}
      <div class="flex items-center gap-1.5">
        {/* Velocity */}
        <Show when={rightVelLabel() !== null}>
          <div class="flex items-center gap-0.5">
            <span class="text-green-400 font-black text-[9px]">R</span>
            <span class="text-zinc-300 font-bold">{rightVelLabel()}</span>
            {modBadge(props.rightModifier())}
          </div>
        </Show>

        {/* Previous octave (N) */}
        <button
          onClick={() => props.onRightOctaveChange(-1)}
          class="flex items-center justify-center w-6 h-6 rounded border border-zinc-700 bg-zinc-900 text-zinc-400 hover:border-zinc-500 hover:text-zinc-200 text-[9px] font-bold transition-colors select-none"
          title="Right hand: previous octave (N)"
        >
          N
        </button>

        {/* Octave badge */}
        <div class="flex items-center justify-center w-8 h-6 rounded border border-green-600 bg-green-950 select-none">
          <span class="text-[0.8rem] font-black text-green-300 tabular-nums leading-none">
            {octaveLabel(props.rightOctave())}
          </span>
        </div>

        {/* Next octave (H) */}
        <button
          onClick={() => props.onRightOctaveChange(1)}
          class="flex items-center justify-center w-6 h-6 rounded border border-zinc-700 bg-zinc-900 text-zinc-400 hover:border-zinc-500 hover:text-zinc-200 text-[9px] font-bold transition-colors select-none"
          title="Right hand: next octave (H)"
        >
          H
        </button>

        <span class="text-[8px] text-zinc-600 font-bold uppercase tracking-widest w-3">
          R
        </span>
      </div>
    </div>
  );
}
