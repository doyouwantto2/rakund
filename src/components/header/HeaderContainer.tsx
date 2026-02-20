import { For } from "solid-js";
import type { InstrumentInfo, LayerRange } from "../../hooks/usePiano";
import type { Modifier, SectionNum } from "../../utils/keyMapping";
import InstrumentSelect from "./InstrumentSelect";
import LayerIndicator from "./LayerIndicator";

interface HeaderContainerProps {
  currentInstrument: () => InstrumentInfo | null;
  availableInstruments: () => InstrumentInfo[];
  isLoading: () => boolean;
  loadProgress: () => number | null;
  activeFolder: () => string | null;
  onSelectInstrument: (folder: string) => void;
  leftSection: () => SectionNum | null;
  rightSection: () => SectionNum | null;
  leftModifier: () => Modifier;
  rightModifier: () => Modifier;
  leftLayerIdx: () => number;
  rightLayerIdx: () => number;
  availableLayers: () => string[];
  availableLayerRanges: () => LayerRange[];
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

export default function HeaderContainer(props: HeaderContainerProps) {
  return (
    <header class="w-full bg-zinc-900 border-b border-zinc-800 px-4 py-3">
      {/*
        3-column grid:
          [auto]  — InstrumentSelect (shrinks to content)
          [1fr]   — section buttons (centred, never shifts)
          [auto]  — LayerIndicator (shrinks to content, right-aligned)
      */}
      <div class="grid grid-cols-[auto_1fr_auto] items-center gap-4">

        {/* ── Left: instrument selector ── */}
        <InstrumentSelect
          currentInstrument={props.currentInstrument}
          availableInstruments={props.availableInstruments}
          isLoading={props.isLoading}
          loadProgress={props.loadProgress}
          activeFolder={props.activeFolder}
          onSelectInstrument={props.onSelectInstrument}
        />

        {/* ── Center: hand section buttons ── */}
        <div class="flex items-center justify-center gap-2">
          {/* Left hand */}
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

          {/* Right hand */}
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
        </div>

        {/* ── Right: layer indicator ── */}
        <LayerIndicator
          layerRanges={props.availableLayerRanges}
          leftLayerIdx={props.leftLayerIdx}
          rightLayerIdx={props.rightLayerIdx}
          leftActive={() => props.leftSection() !== null}
          rightActive={() => props.rightSection() !== null}
        />

      </div>
    </header>
  );
}
