import { createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { getKeyToMidi } from "../utils/keyMapping";

export function usePiano() {
  const [activeNotes, setActiveNotes] = createSignal(new Set<number>());
  const [selectedLayer, setSelectedLayer] = createSignal("MF");
  
  const layers = ["PP", "MP", "MF", "FF"];

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
      const midi = getKeyToMidi(e.key.toLowerCase());
      if (midi && !e.repeat) noteOn(midi);
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      const midi = getKeyToMidi(e.key.toLowerCase());
      if (midi) noteOff(midi);
    };
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  });

  return {
    activeNotes,
    selectedLayer,
    setSelectedLayer,
    layers,
    noteOn,
    noteOff,
    getKeyToMidi
  };
}
