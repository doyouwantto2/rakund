import { createSignal, For } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

export default function Piano() {
  const [activeNotes, setActiveNotes] = createSignal(new Set<number>());
  // PP is now your default
  const [selectedLayer, setSelectedLayer] = createSignal("PP");

  const layers = ["MP", "PP", "MF", "FF"];

  const whiteKeys = Array.from({ length: 88 }, (_, i) => i + 21).filter(
    (m) => ![1, 3, 6, 8, 10].includes(m % 12)
  );

  const isBlack = (midi: number) => [1, 3, 6, 8, 10].includes(midi % 12);

  const noteOn = async (midi: number) => {
    if (activeNotes().has(midi)) return;
    setActiveNotes(prev => new Set(prev).add(midi));
    try {
      await invoke("play_midi_note", {
        midiNum: midi,
        velocity: 100,
        layer: selectedLayer()
      });
    } catch (error) {
      console.error("Error playing note:", midi, error);
    }
  };

  const noteOff = async (midi: number) => {
    setActiveNotes(prev => {
      const n = new Set(prev);
      n.delete(midi);
      return n;
    });
    try {
      await invoke("stop_midi_note", { midiNum: midi });
    } catch (error) {
      console.error("Error stopping note:", midi, error);
    }
  };

  return (
    <div class="h-screen bg-black text-white flex flex-col overflow-hidden select-none font-sans">
      {/* HEADER SECTION - Now minimal */}
      <div class="p-6 flex items-center justify-between border-b border-zinc-800 bg-zinc-950">
        <div class="flex items-center gap-8">
          <h1 class="text-2xl tracking-tighter uppercase font-black italic text-emerald-500">
            Raku Grand
          </h1>

          <div class="flex items-center gap-2 bg-zinc-900 p-1.5 rounded-xl border border-zinc-800">
            <span class="text-[10px] font-bold text-zinc-500 px-3 uppercase tracking-widest">
              Timbre
            </span>
            <For each={layers}>{(l) => (
              <button
                onClick={() => setSelectedLayer(l)}
                class={`px-4 py-1.5 rounded-lg text-xs font-bold transition-all duration-200 ${selectedLayer() === l
                  ? "bg-emerald-500 text-black shadow-lg shadow-emerald-500/20"
                  : "text-zinc-400 hover:text-white hover:bg-zinc-800"
                  }`}
              >
                {l}
              </button>
            )}</For>
          </div>
        </div>
      </div>

      {/* PIANO SECTION */}
      <div class="flex-1 flex flex-col justify-end bg-zinc-900 p-8">
        <div class="overflow-x-auto pb-8 custom-scrollbar">
          <div class="relative flex mx-auto h-[350px]" style={{ width: `${whiteKeys.length * 44}px` }}>

            {/* White Keys */}
            <For each={Array.from({ length: 88 }, (_, i) => i + 21).filter(m => !isBlack(m))}>{(midi) => (
              <button
                onMouseDown={() => noteOn(midi)}
                onMouseUp={() => noteOff(midi)}
                onMouseLeave={() => noteOff(midi)}
                style={{ width: '44px' }}
                class={`h-full bg-zinc-50 border border-zinc-300 rounded-b-xl transition-all duration-75 flex items-end justify-center pb-4 text-[11px] font-bold text-zinc-400
                  ${activeNotes().has(midi)
                    ? "bg-emerald-300 border-emerald-500 translate-y-2 shadow-[inset_0_0_20px_rgba(0,0,0,0.1)]"
                    : "hover:bg-white active:bg-emerald-200"}`}
              >
                {midi % 12 === 0 ? "C" + (Math.floor(midi / 12) - 1) : ""}
              </button>
            )}</For>

            {/* Black Keys */}
            <For each={Array.from({ length: 88 }, (_, i) => i + 21).filter(m => isBlack(m))}>{(midi) => {
              const whiteIndex = Array.from({ length: midi - 21 }, (_, i) => i + 21).filter(m => !isBlack(m)).length;
              return (
                <button
                  onMouseDown={() => noteOn(midi)}
                  onMouseUp={() => noteOff(midi)}
                  onMouseLeave={() => noteOff(midi)}
                  style={{
                    left: `${whiteIndex * 44 - 14}px`,
                    width: '28px',
                    height: '65%'
                  }}
                  class={`absolute z-20 bg-zinc-950 border-x border-b border-black rounded-b-md shadow-2xl transition-all duration-75
                    ${activeNotes().has(midi)
                      ? "bg-emerald-600 border-emerald-400 translate-y-2 shadow-none"
                      : "hover:bg-zinc-800"}`}
                />
              );
            }}</For>
          </div>
        </div>
      </div>
    </div>
  );
}
