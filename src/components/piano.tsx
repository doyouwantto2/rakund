import { For, createMemo } from "solid-js";
import WhiteKey from "./whitekey";
import BlackKey from "./blackkey";

interface PianoProps {
  activeNotes: () => Set<number>;
  onNoteOn: (midi: number) => void;
  onNoteOff: (midi: number) => void;
}

export default function Piano(props: PianoProps) {
  const { activeNotes, onNoteOn, onNoteOff } = props;

  const allMidiNotes = Array.from({ length: 88 }, (_, i) => i + 21);
  const whiteKeys = allMidiNotes.filter((m) => ![1, 3, 6, 8, 10].includes(m % 12));
  const isBlack = (midi: number) => [1, 3, 6, 8, 10].includes(midi % 12);

  const activeNotesSet = createMemo(() => activeNotes());

  return (
    <div class="w-full pb-4 px-2">
      <div class="relative flex w-full h-[140px] shadow-[0_-20px_50px_rgba(0,0,0,0.5)]">
        {/* White Keys */}
        <For each={whiteKeys}>{(midi) => (
          <WhiteKey
            midi={midi}
            active={() => activeNotesSet().has(midi)}
            onMouseDown={() => onNoteOn(midi)}
            onMouseUp={() => onNoteOff(midi)}
            onMouseLeave={() => onNoteOff(midi)}
          />
        )}</For>

        {/* Black Keys */}
        <For each={allMidiNotes.filter(m => isBlack(m))}>{(midi) => {
          const whiteIndex = allMidiNotes.filter(m => m < midi && !isBlack(m)).length;
          return (
            <BlackKey
              active={() => activeNotesSet().has(midi)}
              onMouseDown={() => onNoteOn(midi)}
              onMouseUp={() => onNoteOff(midi)}
              onMouseLeave={() => onNoteOff(midi)}
              style={`left: calc(${whiteIndex} * 1.923% - 0.55%)`}
            />
          );
        }}</For>
      </div>

      {/* FOOTER LEGEND: Directly under keys */}
      <div class="mt-4 text-center">
        <p class="text-[8px] font-bold text-zinc-800 tracking-[0.4em] uppercase">
          A-L Whites | W-U Blacks
        </p>
      </div>
    </div>
  );
}
