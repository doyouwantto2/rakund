import { createSignal, onMount, createEffect } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { getKeyToMidi } from "../utils/keyMapping";

export function usePiano() {
  const [activeNotes, setActiveNotes] = createSignal(new Set<number>());
  const [selectedInstrument, setSelectedInstrument] = createSignal("splendid"); // Default to splendid
  const [selectedLayer, setSelectedLayer] = createSignal("MF"); // Default to MF
  const [hoveredLayer, setHoveredLayer] = createSignal<string | null>(null);
  const [isLayerLoading, setIsLayerLoading] = createSignal(false);
  const [availableLayers, setAvailableLayers] = createSignal<string[]>(["PP", "MP", "MF", "FF"]);

  // Load layer when instrument or layer changes
  const loadLayer = async (instrument: string, layer: string) => {
    setIsLayerLoading(true);
    try {
      await invoke("load_instrument_layer", { instrument, layer });
      console.log(`[CACHE] Loaded ${layer} for ${instrument}`);
    } catch (e) {
      console.error("[CACHE] Failed to load layer:", e);
    } finally {
      setIsLayerLoading(false);
    }
  };

  // Get instrument info and layers
  const loadInstrumentInfo = async (instrument: string) => {
    try {
      const info = await invoke<{ name: string; layers: string[] }>("get_instrument_info", { instrument });
      setAvailableLayers(info.layers);
      console.log(`[INSTRUMENT] Loaded info for ${instrument}:`, info);
    } catch (e) {
      console.error("[INSTRUMENT] Failed to load instrument info:", e);
      // Fallback to default layers
      setAvailableLayers(instrument === "splendid" ? ["PP", "MP", "MF", "FF"] :
        Array.from({ length: 16 }, (_, i) => `V${String(i + 1).padStart(2, '0')}`));
    }
  };

  // Auto-load layer when selection changes
  createEffect(() => {
    const instrument = selectedInstrument();
    const layer = selectedLayer();
    if (instrument && layer) {
      loadLayer(instrument, layer);
    }
  });

  // Load instrument info when instrument changes
  createEffect(() => {
    const instrument = selectedInstrument();
    if (instrument) {
      loadInstrumentInfo(instrument);
      // Set to default MF layer for new instruments
      if (instrument === "splendid") {
        setSelectedLayer("MF");
      } else if (instrument === "salamander") {
        setSelectedLayer("V07"); // Medium velocity for salamander
      }
    }
  });

  // Get appropriate velocity for the selected layer
  const getVelocityForLayer = (instrument: string, layer: string): number => {
    if (instrument === "splendid") {
      switch (layer) {
        case "PP": return 20;  // Very soft (1-40 range)
        case "MP": return 54;  // Medium soft (41-67 range) 
        case "MF": return 76;  // Medium hard (68-84 range)
        case "FF": return 106; // Very hard (85-127 range)
        default: return 100;
      }
    } else if (instrument === "salamander") {
      // Extract velocity from V01, V02, etc.
      if (layer.startsWith("V")) {
        const num = parseInt(layer.substring(1));
        if (num <= 2) return 30;      // V01-V02: soft
        if (num <= 4) return 50;      // V03-V04: medium soft  
        if (num <= 6) return 75;      // V05-V06: medium
        if (num <= 8) return 100;     // V07-V08: medium hard
        if (num <= 12) return 115;    // V09-V12: hard
        return 127;                   // V13-V16: very hard
      }
    }
    return 100; // Default
  };

  const noteOn = async (midi: number) => {
    if (activeNotes().has(midi)) return;
    setActiveNotes(prev => new Set(prev).add(midi));

    console.log(`[FRONTEND] Playing note ${midi}, instrument: ${selectedInstrument()}, layer: ${selectedLayer()}`);

    try {
      await invoke("play_midi_note", {
        midiNum: midi,
        velocity: getVelocityForLayer(selectedInstrument(), selectedLayer()),
        instrument: selectedInstrument(),
        layer: selectedLayer()
      });
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
    selectedInstrument,
    setSelectedInstrument,
    selectedLayer,
    setSelectedLayer,
    hoveredLayer,
    setHoveredLayer,
    isLayerLoading,
    availableLayers,
    noteOn,
    noteOff,
    getKeyToMidi
  };
}
