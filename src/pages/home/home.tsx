import { createSignal, onMount, For } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

export default function Piano() {
  const [activeNotes, setActiveNotes] = createSignal(new Set<number>());
  const [selectedLayer, setSelectedLayer] = createSignal("PP");
  const layers = ["MP", "PP", "MF", "FF"];

  const keyMap: Record<string, number> = {
    'a': 60, 'w': 61, 's': 62, 'e': 63, 'd': 64, 'f': 65, 't': 66,
    'g': 67, 'y': 68, 'h': 69, 'u': 70, 'j': 71, 'k': 72, 'o': 73, 'l': 74
  };

  const allMidiNotes = Array.from({ length: 88 }, (_, i) => i + 21);
  const whiteKeys = allMidiNotes.filter((m) => ![1, 3, 6, 8, 10].includes(m % 12));
  const isBlack = (midi: number) => [1, 3, 6, 8, 10].includes(midi % 12);

  const noteOn = async (midi: number) => {
    if (activeNotes().has(midi)) return;
    setActiveNotes(prev => new Set(prev).add(midi));
    
    // Map layer to velocity
    const layerVelocity: Record<string, number> = {
      "PP": 20,  // Very soft (1-40 range)
      "MP": 54,  // Medium soft (41-67 range)  
      "MF": 76,  // Medium hard (68-84 range)
      "FF": 106  // Very hard (85-127 range)
    };
    
    const velocity = layerVelocity[selectedLayer()] || 100;
    
    try {
      await invoke("play_midi_note", { midiNum: midi, velocity: velocity, layer: selectedLayer() });
    } catch (e) { console.error(e); }
  };

  const noteOff = async (midi: number) => {
    setActiveNotes(prev => {
      const n = new Set(prev);
      n.delete(midi);
      return n;
    });
    try {
      await invoke("stop_midi_note", { midiNum: midi });
    } catch (e) { console.error(e); }
  };

  onMount(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const midi = keyMap[e.key.toLowerCase()];
      if (midi && !e.repeat) noteOn(midi);
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      const midi = keyMap[e.key.toLowerCase()];
      if (midi) noteOff(midi);
    };
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  });

  return (
    <div class="h-screen w-full bg-zinc-950 text-white flex flex-col justify-between overflow-hidden select-none font-sans">
      {/* HEADER: Sticks to top */}
      <div class="shrink-0 p-4 flex items-center justify-between border-b border-zinc-900 bg-black/40 backdrop-blur-sm">
        <div class="flex items-center gap-6">
          <h1 class="text-xl font-black italic text-emerald-500 tracking-tighter uppercase leading-none">Raku Grand</h1>
          <div class="flex items-center gap-1 bg-zinc-900 p-1 rounded-lg border border-zinc-800">
            <For each={layers}>{(l) => (
              <button
                onClick={() => setSelectedLayer(l)}
                class={`px-3 py-1 rounded text-[9px] font-bold transition-all ${selectedLayer() === l ? "bg-emerald-500 text-black shadow-md" : "text-zinc-500 hover:text-zinc-200"
                  }`}
              >
                {l}
              </button>
            )}</For>
          </div>
        </div>
        <div class="text-[9px] font-bold tracking-widest text-emerald-500/50 uppercase border border-emerald-500/20 px-3 py-1 rounded-full">
          Resonance Active
        </div>
      </div>

      {/* SPACER: Pushes the piano down */}
      <div class="flex-1 flex items-center justify-center opacity-20 pointer-events-none">
        <div class="text-6xl font-black italic text-zinc-800 tracking-tighter uppercase">Raku</div>
      </div>

      {/* PIANO AREA: Positioned at the bottom */}
      <div class="w-full pb-4 px-2">
        <div class="relative flex w-full h-[160px] shadow-[0_-20px_50px_rgba(0,0,0,0.5)]">
          {/* White Keys */}
          <For each={whiteKeys}>{(midi) => (
            <button
              onMouseDown={() => noteOn(midi)}
              onMouseUp={() => noteOff(midi)}
              onMouseLeave={() => noteOff(midi)}
              class={`h-full w-[1.923%] shrink-0 bg-zinc-200 border-x border-zinc-400 rounded-b-sm flex items-end justify-center pb-2 text-[7px] font-black text-zinc-500 transition-all duration-75
                ${activeNotes().has(midi) ? "!bg-emerald-400 !border-emerald-500 translate-y-1" : "hover:bg-white"}`}
            >
              {midi % 12 === 0 ? "C" + (Math.floor(midi / 12) - 1) : ""}
            </button>
          )}</For>

          {/* Black Keys */}
          <For each={allMidiNotes.filter(m => isBlack(m))}>{(midi) => {
            const whiteIndex = allMidiNotes.filter(m => m < midi && !isBlack(m)).length;
            return (
              <button
                onMouseDown={() => noteOn(midi)}
                onMouseUp={() => noteOff(midi)}
                onMouseLeave={() => noteOff(midi)}
                class={`absolute z-20 h-[58%] w-[1.1%] bg-zinc-900 border-x border-b border-black rounded-b-sm transition-all duration-75
                  ${activeNotes().has(midi) ? "!bg-emerald-700 !border-emerald-500 translate-y-1" : "hover:bg-zinc-800"}`}
                style={{ left: `calc(${whiteIndex} * 1.923% - 0.55%)` }}
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
    </div>
  );
}
