import { createMemo, For } from "solid-js";
import { getKeyHighlight, type SectionNum } from "../utils/keyMapping";
import { usePiano } from "../hooks/usePiano";

interface PianoProps {
  activeNotes: () => Set<number>;
  onNoteOn: (midi: number) => void;
  onNoteOff: (midi: number) => void;
  leftSection: () => SectionNum | null;
  rightSection: () => SectionNum | null;
}

const LEFT_LABELS: Record<SectionNum, string> = { 1: "b · A0–E2", 2: "g · A1–E3", 3: "t · A2–E4" };
const RIGHT_LABELS: Record<SectionNum, string> = { 1: "n · F4–C6", 2: "h · F5–C7", 3: "y · F6–C8" };

const allMidi = Array.from({ length: 88 }, (_, i) => i + 21);
const isBlack = (midi: number) => [1, 3, 6, 8, 10].includes(midi % 12);
const whiteKeys = allMidi.filter(m => !isBlack(m));
const blackKeys = allMidi.filter(m => isBlack(m));

// Precompute black key left-offset (doesn't change)
const blackKeyLeftPct = new Map<number, number>(
  blackKeys.map(midi => [midi, allMidi.filter(m => m < midi && !isBlack(m)).length])
);

export default function Piano(props: PianoProps) {
  const { activeNotes, onNoteOn, onNoteOff, leftSection, rightSection } = props;
  const { altPressed } = usePiano();

  // Reactive highlight map — recomputes when sections change
  const highlights = createMemo(() => {
    const map = new Map<number, 'left' | 'right'>();
    for (const midi of allMidi) {
      const h = getKeyHighlight(midi, leftSection(), rightSection());
      if (h) map.set(midi, h);
    }
    return map;
  });

  // Reactive helpers — called inside JSX so SolidJS tracks them
  const whiteClass = (midi: number) => {
    const active = activeNotes().has(midi);
    const highlight = highlights().get(midi);
    if (active) return "bg-emerald-400 border-emerald-500 translate-y-1 shadow-lg";
    if (highlight === 'left') return "bg-blue-300 border-blue-400";
    if (highlight === 'right') return "bg-green-300 border-green-400";
    return "bg-zinc-100 hover:bg-white border-zinc-400";
  };

  const blackClass = (midi: number) => {
    const active = activeNotes().has(midi);
    const highlight = highlights().get(midi);
    if (active) return "bg-emerald-500 translate-y-1 shadow-lg";
    if (highlight === 'left') return "bg-blue-600";
    if (highlight === 'right') return "bg-green-700";
    return "bg-zinc-900 hover:bg-zinc-700";
  };

  return (
    <div class="w-full pb-4 px-2">

      {/* Section indicator */}
      <div class="flex gap-4 justify-center mb-3 text-xs font-bold">
        <div class="flex items-center gap-1">
          <span class="text-zinc-500">Left:</span>
          <For each={[1, 2, 3] as SectionNum[]}>{s => (
            <span class={`px-2 py-0.5 rounded border transition-all duration-150 ${leftSection() === s
              ? "bg-blue-600 border-blue-400 text-white"
              : "bg-zinc-800 border-zinc-700 text-zinc-500"
              }`}>
              {LEFT_LABELS[s]}
            </span>
          )}</For>
        </div>

        <div class="w-px bg-zinc-700" />

        <div class="flex items-center gap-1">
          <span class="text-zinc-500">Right:</span>
          <For each={[1, 2, 3] as SectionNum[]}>{s => (
            <span class={`px-2 py-0.5 rounded border transition-all duration-150 ${rightSection() === s
              ? "bg-green-600 border-green-400 text-white"
              : "bg-zinc-800 border-zinc-700 text-zinc-500"
              }`}>
              {RIGHT_LABELS[s]}
            </span>
          )}</For>
        </div>

        <div class="w-px bg-zinc-700" />

        <div class="flex items-center gap-2 text-zinc-600">
          <span class={`px-2 py-0.5 rounded border text-[10px] font-bold ${altPressed() ? "bg-blue-900 border-blue-500 text-blue-300" : "border-zinc-800 text-zinc-700"}`}>Alt</span>
          <span class="w-3 h-3 rounded-sm bg-blue-400 inline-block" />Left
          <span class="w-3 h-3 rounded-sm bg-green-400 inline-block" />Right
          <span class="text-zinc-700">· ESC clears</span>
        </div>
      </div>

      {/* Keys */}
      <div class="relative flex w-full h-[150px] shadow-[0_-20px_50px_rgba(0,0,0,0.6)]">

        {/* White keys — use For so each re-renders reactively */}
        <For each={whiteKeys}>{midi => (
          <button
            onMouseDown={() => onNoteOn(midi)}
            onMouseUp={() => onNoteOff(midi)}
            onMouseLeave={() => onNoteOff(midi)}
            class={`h-full w-[1.923%] shrink-0 border-x rounded-b-sm
              flex items-end justify-center pb-1 text-[7px] font-black text-zinc-500
              transition-all duration-75 ${whiteClass(midi)}`}
          >
            {midi % 12 === 0 ? `C${Math.floor(midi / 12) - 1}` : ""}
          </button>
        )}</For>

        {/* Black keys */}
        <For each={blackKeys}>{midi => (
          <button
            onMouseDown={() => onNoteOn(midi)}
            onMouseUp={() => onNoteOff(midi)}
            onMouseLeave={() => onNoteOff(midi)}
            class={`absolute top-0 h-[60%] w-[1.2%] rounded-b-sm z-10
              transition-all duration-75 ${blackClass(midi)}`}
            style={`left: calc(${blackKeyLeftPct.get(midi)} * 1.923% + 0.96% - 0.6%)`}
          />
        )}</For>

      </div>

      <div class="mt-2 text-center text-[8px] text-zinc-700 tracking-[0.3em] uppercase font-bold">
        Left: z x c v a s d f q w e r &nbsp;·&nbsp; Right: m , . / j k l ; u i o p
      </div>
    </div>
  );
}
