import { createSignal, onMount, For } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

export default function Piano() {
  const [activeNotes, setActiveNotes] = createSignal(new Set<number>());
  const [pedal, setPedal] = createSignal(false);

  const keys = Array.from({ length: 88 }, (_, i) => {
    const midi = i + 21;
    const isBlack = [1, 3, 6, 8, 10].includes(midi % 12);
    return { midi, isBlack };
  });

  const noteOn = async (midi: number) => {
    setActiveNotes(prev => new Set(prev).add(midi));
    try {
      await invoke("play_midi_note", { midiNum: midi, velocity: 100 });
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

  onMount(() => {
    const handleKey = (e: KeyboardEvent, down: boolean) => {
      if (e.code === "Space") {
        e.preventDefault();
        setPedal(down);
        invoke("set_sustain", { active: down });
      }
    };
    window.addEventListener("keydown", (e) => handleKey(e, true));
    window.addEventListener("keyup", (e) => handleKey(e, false));
  });

  return (
    <div class="h-screen bg-black text-white flex flex-col overflow-hidden select-none">
      <div class="p-4 flex justify-between border-b border-zinc-800">
        <h1 class="text-xl tracking-tighter uppercase font-bold">Raku Grand</h1>
        <div class={`px-4 py-1 rounded-full text-xs font-bold transition-colors ${pedal() ? "bg-emerald-500 text-black" : "bg-zinc-800 text-zinc-500"}`}>
          SUSTAIN PEDAL (SPACE)
        </div>
      </div>

      <div class="flex-1 flex items-center overflow-x-auto p-12 bg-zinc-900">
        <div class="relative flex mx-auto border-t-[10px] border-zinc-800">
          <For each={keys}>{(key) => (
            <div class="relative">
              <button
                onMouseDown={() => noteOn(key.midi)}
                onMouseUp={() => noteOff(key.midi)}
                onMouseLeave={() => noteOff(key.midi)}
                class={`${key.isBlack
                  ? "absolute z-10 w-6 h-32 -translate-x-1/2 bg-zinc-950 border border-black rounded-b-sm"
                  : "w-10 h-52 bg-white border border-zinc-300 rounded-b-md"} 
                  ${activeNotes().has(key.midi) ? "!bg-emerald-400" : ""}`}
              />
            </div>
          )}</For>
        </div>
      </div>
    </div>
  );
}
