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
      <div class="flex items-center justify-between gap-4">

        {/* ── Left: instrument selector ── */}
        <InstrumentSelect
          currentInstrument={props.currentInstrument}
          availableInstruments={props.availableInstruments}
          isLoading={props.isLoading}
          loadProgress={props.loadProgress}
          activeFolder={props.activeFolder}
          onSelectInstrument={props.onSelectInstrument}
        />

        {/* ── Right: section buttons + layer indicator ── */}
        <div class="flex items-center gap-3">

          {/* Left hand sections */}
          <div class="flex items-center gap-1">
            <span class="text-[9px] text-zinc-600 font-bold uppercase tracking-widest mr-0.5">L</span>
            <For each={LEFT_SECTIONS}>{({ num, key, label }) => (
              <div class={`flex items-center gap-1 px-2 py-0.5 rounded border transition-all duration-100 bg-zinc-900 ${props.leftSection() === num
                ? "border-blue-400" // Blue border when active
                : "border-zinc-800"
                }`}>
                <kbd class={`text-[9px] font-black rounded px-0.5 ${props.leftSection() === num ? "text-blue-300" : "text-zinc-600"
                  }`}>{key.toUpperCase()}</kbd>
                <span class={`text-[9px] font-bold ${props.leftSection() === num ? "text-blue-400" : "text-zinc-700"
                  }`}>{label}</span>
              </div>
            )}</For>
          </div>

          <div class="h-4 w-px bg-zinc-800" />

          {/* Right hand sections */}
          <div class="flex items-center gap-1">
            <For each={RIGHT_SECTIONS}>{({ num, key, label }) => (
              <div class={`flex items-center gap-1 px-2 py-0.5 rounded border transition-all duration-100 bg-zinc-900 ${props.rightSection() === num
                ? "border-green-400" // Green border when active
                : "border-zinc-800"
                }`}>
                <kbd class={`text-[9px] font-black rounded px-0.5 ${props.rightSection() === num ? "text-green-300" : "text-zinc-600"
                  }`}>{key.toUpperCase()}</kbd>
                <span class={`text-[9px] font-bold ${props.rightSection() === num ? "text-green-400" : "text-zinc-700"
                  }`}>{label}</span>
              </div>
            )}</For>
            <span class="text-[9px] text-zinc-600 font-bold uppercase tracking-widest ml-0.5">R</span>
          </div>

          <div class="h-4 w-px bg-zinc-800" />

          {/* Layer indicator */}
          <LayerIndicator
            layerRanges={props.availableLayerRanges}
            leftLayerIdx={props.leftLayerIdx}
            rightLayerIdx={props.rightLayerIdx}
            leftActive={() => props.leftSection() !== null}
            rightActive={() => props.rightSection() !== null}
          />

        </div>
      </div>
    </header>
  );
}
