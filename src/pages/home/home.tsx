import { createSignal, onMount, For } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

export default function Piano() {
  const [activeNotes, setActiveNotes] = createSignal(new Set<number>());
  const [pedal, setPedal] = createSignal(false);
  const [selectedLayer, setSelectedLayer] = createSignal("Auto"); // Default to Auto-velocity

  const layers = ["MP", "PP", "MF", "FF"];

  // Helper: Total white keys determines the container width
  const whiteKeys = Array.from({ length: 88 }, (_, i) => i + 21).filter(
    (m) => ![1, 3, 6, 8, 10].includes(m % 12)
  );

  const isBlack = (midi: number) => [1, 3, 6, 8, 10].includes(midi % 12);

  const noteOn = async (midi: number) => {
    if (activeNotes().has(midi)) return; // Prevent repeat triggers
    setActiveNotes(prev => new Set(prev).add(midi));
    try {
      // Send the layer to Rust
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

  // ... (Keep your onMount sustain logic here)

  return (
    <div class="h-screen bg-black text-white flex flex-col overflow-hidden select-none font-sans">
      {/* HEADER SECTION */}
      <div class="p-4 flex items-center justify-between border-b border-zinc-800 bg-zinc-950">
        <div class="flex items-center gap-6">
          <h1 class="text-xl tracking-tighter uppercase font-black italic">Raku Grand</h1>

          {/* LAYER SELECTOR */}
          <div class="flex items-center gap-2 bg-zinc-900 p-1 rounded-lg border border-zinc-800">
            <span class="text-[10px] font-bold text-zinc-500 px-2 uppercase">Layer Mode</span>
            <For each={layers}>{(l) => (
              <button
                onClick={() => setSelectedLayer(l)}
                class={`px-3 py-1 rounded-md text-xs font-bold transition-all ${selectedLayer() === l ? "bg-emerald-500 text-black" : "text-zinc-400 hover:text-white"
                  }`}
              >
                {l}
              </button>
            )}</For>
          </div>
        </div>

        <div class={`px-4 py-1 rounded-full text-xs font-bold transition-colors ${pedal() ? "bg-emerald-500 text-black shadow-[0_0_15px_rgba(16,185,129,0.4)]" : "bg-zinc-800 text-zinc-500"
          }`}>
          SUSTAIN (SPACE)
        </div>
      </div>

      {/* PIANO SECTION */}
      <div class="flex-1 flex flex-col justify-end bg-gradient-to-b from-zinc-900 to-black p-4">
        {/* Scrollable wrapper to allow zooming/viewing full keyboard */}
        <div class="overflow-x-auto pb-4 custom-scrollbar">
          <div class="relative flex mx-auto h-[300px]" style={{ width: `${whiteKeys.length * 40}px` }}>

            {/* 1. RENDER WHITE KEYS FIRST */}
            <For each={Array.from({ length: 88 }, (_, i) => i + 21).filter(m => !isBlack(m))}>{(midi) => (
              <button
                onMouseDown={() => noteOn(midi)}
                onMouseUp={() => noteOff(midi)}
                onMouseLeave={() => noteOff(midi)}
                style={{ width: '40px' }}
                class={`h-full bg-zinc-50 border border-zinc-300 rounded-b-lg transition-colors duration-75 flex items-end justify-center pb-2 text-[10px] text-zinc-400
                  ${activeNotes().has(midi) ? "!bg-emerald-400 !border-emerald-500 translate-y-1 shadow-inner" : ""}`}
              >
                {midi % 12 === 0 ? "C" + (Math.floor(midi / 12) - 1) : ""}
              </button>
            )}</For>

            {/* 2. RENDER BLACK KEYS (ABSOLUTE) */}
            <For each={Array.from({ length: 88 }, (_, i) => i + 21).filter(m => isBlack(m))}>{(midi) => {
              // Calculation to find horizontal position based on the previous white keys
              const whiteIndex = Array.from({ length: midi - 21 }, (_, i) => i + 21).filter(m => !isBlack(m)).length;
              return (
                <button
                  onMouseDown={() => noteOn(midi)}
                  onMouseUp={() => noteOff(midi)}
                  onMouseLeave={() => noteOff(midi)}
                  style={{
                    left: `${whiteIndex * 40 - 12}px`,
                    width: '24px',
                    height: '60%'
                  }}
                  class={`absolute z-20 bg-zinc-900 border-x border-b border-black rounded-b shadow-xl transition-all
                      ${activeNotes().has(midi) ? "!bg-emerald-600 border-emerald-400 translate-y-1" : "hover:bg-zinc-800"}`}
                />
              );
            }}</For>
          </div>
        </div>
      </div>
    </div>
  );
}
