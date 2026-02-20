import { For, Show } from "solid-js";
import type { InstrumentInfo } from "../../hooks/usePiano";
import type { Modifier, SectionNum } from "../../utils/keyMapping";
import InstrumentSelect from "./InstrumentSelect";
import VolumeControl from "./VolumeControl";

interface HeaderContainerProps {
  currentInstrument: () => InstrumentInfo | null;
  availableInstruments: () => InstrumentInfo[];
  isLoading: () => boolean;
  loadProgress: () => number | null;
  activeFolder: () => string | null;
  onSelectInstrument: (folder: string) => void;
  volume: () => number;
  onVolumeChange: (v: number) => void;
  leftSection: () => SectionNum | null;
  rightSection: () => SectionNum | null;
  leftModifier: () => Modifier;
  rightModifier: () => Modifier;
  leftLayerIdx: () => number;
  rightLayerIdx: () => number;
  availableLayers: () => string[];
  velocityForLayer: (layer: string) => number;
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

function modBadge(mod: Modifier) {
  if (!mod) return null;
  return (
    <span class={`text-[9px] font-black ml-1 ${mod === 'sharp' ? 'text-emerald-400' : 'text-blue-400'}`}>
      {mod === 'sharp' ? '♯' : '♭'}
    </span>
  );
}

export default function HeaderContainer(props: HeaderContainerProps) {
  const isActive = () => props.leftSection() !== null || props.rightSection() !== null;

  // Use velocityForLayer from usePiano — correctly handles any instrument's layer names
  const leftVelLabel = () => {
    const layers = props.availableLayers();
    if (layers.length === 0) return "—";
    const layer = layers[Math.min(props.leftLayerIdx(), layers.length - 1)];
    const vel = props.velocityForLayer(layer);
    return `${layer} ${vel}`;
  };

  const rightVelLabel = () => {
    const layers = props.availableLayers();
    if (layers.length === 0) return "—";
    const layer = layers[Math.min(props.rightLayerIdx(), layers.length - 1)];
    const vel = props.velocityForLayer(layer);
    return `${layer} ${vel}`;
  };

  return (
    <header class="w-full bg-zinc-900 border-b border-zinc-800 px-4 py-3">
      <div class="flex items-center justify-between gap-4">
        {/* Instrument Selection - Left */}
        <InstrumentSelect
          currentInstrument={props.currentInstrument}
          availableInstruments={props.availableInstruments}
          isLoading={props.isLoading}
          loadProgress={props.loadProgress}
          activeFolder={props.activeFolder}
          onSelectInstrument={props.onSelectInstrument}
        />

        {/* Piano Status Controls - Middle */}
        <div class="flex-1 flex items-center justify-center gap-2 px-4 flex-wrap">
          {/* Left hand sections */}
          <div class="flex items-center gap-1">
            <span class="text-[9px] text-zinc-600 font-bold uppercase tracking-widest mr-0.5">L</span>
            <For each={LEFT_SECTIONS}>{({ num, key, label }) => (
              <div class={`flex items-center gap-1 px-2 py-0.5 rounded border transition-all duration-100 ${props.leftSection() === num
                ? "bg-blue-600/80 border-blue-400"
                : "bg-zinc-900 border-zinc-800"
                }`}>
                <kbd class={`text-[9px] font-black rounded px-0.5 ${props.leftSection() === num ? "text-blue-100" : "text-zinc-600"
                  }`}>{key.toUpperCase()}</kbd>
                <span class={`text-[9px] font-bold ${props.leftSection() === num ? "text-white" : "text-zinc-700"
                  }`}>{label}</span>
              </div>
            )}</For>
          </div>

          <div class="h-4 w-px bg-zinc-800" />

          {/* Right hand sections */}
          <div class="flex items-center gap-1">
            <For each={RIGHT_SECTIONS}>{({ num, key, label }) => (
              <div class={`flex items-center gap-1 px-2 py-0.5 rounded border transition-all duration-100 ${props.rightSection() === num
                ? "bg-green-600/80 border-green-400"
                : "bg-zinc-900 border-zinc-800"
                }`}>
                <kbd class={`text-[9px] font-black rounded px-0.5 ${props.rightSection() === num ? "text-green-100" : "text-zinc-600"
                  }`}>{key.toUpperCase()}</kbd>
                <span class={`text-[9px] font-bold ${props.rightSection() === num ? "text-white" : "text-zinc-700"
                  }`}>{label}</span>
              </div>
            )}</For>
            <span class="text-[9px] text-zinc-600 font-bold uppercase tracking-widest ml-0.5">R</span>
          </div>

          <div class="h-4 w-px bg-zinc-800" />

          {/* Per-hand velocity — only shown when layers are loaded */}
          <Show when={props.availableLayers().length > 0}>
            <div class="flex items-center gap-3 text-[10px]">
              <div class="flex items-center gap-1">
                <span class="text-blue-500 font-black">L</span>
                <span class="text-zinc-300 font-bold">{leftVelLabel()}</span>
                {modBadge(props.leftModifier())}
              </div>
              <div class="flex items-center gap-1">
                <span class="text-green-500 font-black">R</span>
                <span class="text-zinc-300 font-bold">{rightVelLabel()}</span>
                {modBadge(props.rightModifier())}
              </div>
            </div>

            <div class="h-4 w-px bg-zinc-800" />
          </Show>

          {/* ESC / active status */}
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

        {/* Volume Control - Right */}
        <VolumeControl
          volume={props.volume}
          onVolumeChange={props.onVolumeChange}
        />
      </div>
    </header>
  );
}
