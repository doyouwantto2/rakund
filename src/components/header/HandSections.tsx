import { For, Show } from "solid-js";
import type { SectionNum, Modifier } from "../../utils/keyMapping";

interface HandSectionsProps {
  leftSection: () => SectionNum | null;
  rightSection: () => SectionNum | null;
  leftModifier: () => Modifier;
  rightModifier: () => Modifier;
  leftLayerIdx: () => number;
  rightLayerIdx: () => number;
  availableLayers: () => string[];
  velocityForLayer: (layer: string) => number;
  setLeftSection: (section: SectionNum | null) => void;
  setRightSection: (section: SectionNum | null) => void;
}

const LEFT_SECTIONS: { num: SectionNum; key: string; label: string }[] = [
  { num: 1, key: "B", label: "F0–C2" },
  { num: 2, key: "G", label: "F1–C3" },
  { num: 3, key: "T", label: "F2–C4" },
];

const RIGHT_SECTIONS: { num: SectionNum; key: string; label: string }[] = [
  { num: 1, key: "N", label: "F3–C5" },
  { num: 2, key: "H", label: "F4–C6" },
  { num: 3, key: "Y", label: "F5–C7" },
];

function modBadge(mod: Modifier) {
  if (!mod) return null;
  return (
    <span class={`text-[8px] font-black ml-1.5 ${mod === 'sharp' ? 'text-emerald-400' : 'text-blue-400'}`}>
      {mod === 'sharp' ? '♯' : '♭'}
    </span>
  );
}

export default function HandSections(props: HandSectionsProps) {
  const isActive = () => props.leftSection() !== null || props.rightSection() !== null;

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

  return (
    <div class="flex items-center gap-3 text-[10px]">

      {/* Left hand — B / G / T */}
      <div class="flex items-center gap-1">
        <span class="text-[8px] text-zinc-600 font-bold uppercase tracking-widest mr-0.5 w-3">L</span>
        <For each={LEFT_SECTIONS}>{section => (
          <button
            onClick={() => props.setLeftSection(section.num)}
            class={`flex flex-col items-center justify-center w-12 h-7 rounded border text-[9px] font-bold cursor-pointer select-none transition-colors duration-75
              ${props.leftSection() === section.num
                ? "bg-blue-600 border-blue-500 text-white"
                : "bg-zinc-900 border-zinc-700 text-zinc-500 hover:border-zinc-500 hover:text-zinc-300"
              }`}
          >
            <span class="text-[7px] leading-none opacity-50 font-black">{section.key}</span>
            <span class="leading-none mt-0.5">{section.label}</span>
          </button>
        )}</For>
      </div>

      {/* Velocity for left hand — only when a section is active */}
      <Show when={props.leftSection() !== null && leftVelLabel() !== null}>
        <div class="flex items-center gap-0.5 text-[10px]">
          <span class="text-blue-400 font-black">L</span>
          <span class="text-zinc-300 font-bold">{leftVelLabel()}</span>
          {modBadge(props.leftModifier())}
        </div>
      </Show>

      <div class="h-4 w-px bg-zinc-800" />

      {/* Velocity for right hand — only when a section is active */}
      <Show when={props.rightSection() !== null && rightVelLabel() !== null}>
        <div class="flex items-center gap-0.5 text-[10px]">
          <span class="text-green-400 font-black">R</span>
          <span class="text-zinc-300 font-bold">{rightVelLabel()}</span>
          {modBadge(props.rightModifier())}
        </div>
      </Show>

      {/* Right hand — N / H / Y */}
      <div class="flex items-center gap-1">
        <For each={RIGHT_SECTIONS}>{section => (
          <button
            onClick={() => props.setRightSection(section.num)}
            class={`flex flex-col items-center justify-center w-12 h-7 rounded border text-[9px] font-bold cursor-pointer select-none transition-colors duration-75
              ${props.rightSection() === section.num
                ? "bg-green-600 border-green-500 text-white"
                : "bg-zinc-900 border-zinc-700 text-zinc-500 hover:border-zinc-500 hover:text-zinc-300"
              }`}
          >
            <span class="text-[7px] leading-none opacity-50 font-black">{section.key}</span>
            <span class="leading-none mt-0.5">{section.label}</span>
          </button>
        )}</For>
        <span class="text-[8px] text-zinc-600 font-bold uppercase tracking-widest ml-0.5 w-3">R</span>
      </div>

    </div>
  );
}
