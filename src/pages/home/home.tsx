import { createSignal, onMount, For } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

export default function Piano() {
  const [activeNotes, setActiveNotes] = createSignal(new Set<number>());
  const [pedal, setPedal] = createSignal(false);
  const [selectedLayer, setSelectedLayer] = createSignal("PP");
  const layers = ["Auto", "PP", "MF", "FF"];

  const keyMap: Record<string, number> = {
    'a': 60, 'w': 61, 's': 62, 'e': 63, 'd': 64, 'f': 65, 't': 66,
    'g': 67, 'y': 68, 'h': 69, 'u': 70, 'j': 71, 'k': 72, 'o': 73, 'l': 74
  };

  const whiteKeys = Array.from({ length: 88 }, (_, i) => i + 21).filter(
    (m) => ![1, 3, 6, 8, 10].includes(m % 12)
  );

  const isBlack = (midi: number) => [1, 3, 6, 8, 10].includes(midi % 12);

  // Toggle Function for Spacebar
  const togglePedal = async () => {
    const newState = !pedal();
    setPedal(newState);
    try {
      await invoke("set_sustain", { active: newState });
    } catch (e) { console.error(e); }
  };

  const noteOn = async (midi: number) => {
    if (activeNotes().has(midi)) return;
    setActiveNotes(prev => new Set(prev).add(midi));
    try {
      await invoke("play_midi_note", { midiNum: midi, velocity: 100, layer: selectedLayer() });
    } catch (e) { console.error(e); }
  };

  const noteOff = async (midi: number) => {
    setActiveNotes(prev => {
      const n = new Set(prev);
      n.delete(midi);
      return n;
    });
    try { await invoke("stop_midi_note", { midiNum: midi }); } catch (e) { console.error(e); }
  };

  onMount(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Toggle sustain on Space press
      if (e.code === "Space") {
        e.preventDefault();
        if (!e.repeat) togglePedal();
        return;
      }

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
    <div class="h-screen bg-black text-white flex flex-col overflow-hidden select-none font-sans">
      <div class="p-6 flex items-center justify-between border-b border-zinc-800 bg-zinc-950">
        <div class="flex items-center gap-10">
          <h1 class="text-3xl font-black italic text-emerald-500 tracking-tighter uppercase">Raku Grand</h1>

          <div class="flex items-center gap-2 bg-zinc-900 p-1.5 rounded-xl border border-zinc-800">
            <For each={layers}>{(l) => (
              <button
                onClick={() => setSelectedLayer(l)}
                class={`px-5 py-2 rounded-lg text-xs font-bold transition-all ${selectedLayer() === l ? "bg-emerald-500 text-black" : "text-zinc-500"
                  }`}
              >
                {l}
              </button>
            )}</For>
          </div>
        </div>

        <div class="flex items-center gap-4 bg-zinc-900/50 p-3 rounded-2xl border border-zinc-800">
          <div class={`w-3 h-3 rounded-full ${pedal() ? "bg-emerald-500 shadow-[0_0_10px_#10b981]" : "bg-zinc-700"}`} />
          <span class={`text-[11px] font-bold tracking-widest ${pedal() ? "text-emerald-400" : "text-zinc-500"}`}>
            SUSTAIN (SPACE TOGGLE): {pedal() ? "ACTIVE" : "OFF"}
          </span>
        </div>
      </div>

      <div class="flex-1 flex flex-col justify-end bg-zinc-900 p-10">
        <div class="overflow-x-auto pb-10">
          <div class="relative flex mx-auto h-[380px]" style={{ width: `${whiteKeys.length * 46}px` }}>
            <For each={whiteKeys}>{(midi) => (
              <button
                onMouseDown={() => noteOn(midi)}
                onMouseUp={() => noteOff(midi)}
                style={{ width: '46px' }}
                class={`h-full bg-zinc-100 border-x border-zinc-300 rounded-b-2xl transition-all duration-75 flex items-end justify-center pb-6 text-[12px] font-bold text-zinc-400
                  ${activeNotes().has(midi) ? "!bg-emerald-300 !border-emerald-500 translate-y-3" : ""}`}
              >
                {midi % 12 === 0 ? "C" + (Math.floor(midi / 12) - 1) : ""}
              </button>
            )}</For>

            <For each={Array.from({ length: 88 }, (_, i) => i + 21).filter(m => isBlack(m))}>{(midi) => {
              const whiteIndex = Array.from({ length: midi - 21 }, (_, i) => i + 21).filter(m => !isBlack(m)).length;
              return (
                <button
                  onMouseDown={() => noteOn(midi)}
                  onMouseUp={() => noteOff(midi)}
                  style={{ left: `${whiteIndex * 46 - 15}px`, width: '30px', height: '65%' }}
                  class={`absolute z-20 bg-zinc-950 border-x border-b border-black rounded-b-lg transition-all
                    ${activeNotes().has(midi) ? "!bg-emerald-600 border-emerald-400 translate-y-3" : ""}`}
                />
              );
            }}</For>
          </div>
        </div>
      </div>
    </div>
  );
}
