import { createSignal, onMount, createEffect } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getKeyToMidi } from "../utils/keyMapping";

interface LoadProgressEvent {
  layer: string;
  loaded: number;
  total: number;
  progress: number;
}

export function usePiano() {
  const [activeNotes, setActiveNotes] = createSignal(new Set<number>());
  const [selectedLayer, setSelectedLayer] = createSignal("MF"); // Default to MF
  const [hoveredLayer, setHoveredLayer] = createSignal<string | null>(null);
  const [isLayerLoading, setIsLayerLoading] = createSignal(false);
  const [availableLayers, setAvailableLayers] = createSignal<string[]>(["PP", "MP", "MF", "FF"]);
  const [loadProgress, setLoadProgress] = createSignal<{ loaded: number; total: number; progress: number } | null>(null);

  // Load layer when layer changes
  const loadLayer = async (layer: string) => {
    setIsLayerLoading(true);
    setLoadProgress(null);
    try {
      await invoke("load_instrument_layer", { layer });
      console.log(`[CACHE] Loaded ${layer}`);
    } catch (e) {
      console.error("[CACHE] Failed to load layer:", e);
    } finally {
      setIsLayerLoading(false);
      setLoadProgress(null);
    }
  };

  // Get instrument info and layers
  const loadInstrumentInfo = async () => {
    try {
      const info = await invoke<{ name: string; layers: string[] }>("get_instrument_info");
      setAvailableLayers(info.layers);
      console.log(`[INSTRUMENT] Loaded info:`, info);
    } catch (e) {
      console.error("[INSTRUMENT] Failed to load instrument info:", e);
      // Fallback to default layers
      setAvailableLayers(["PP", "MP", "MF", "FF"]);
    }
  };

  // Auto-load layer when selection changes
  createEffect(() => {
    const layer = selectedLayer();
    if (layer) {
      loadLayer(layer);
    }
  });

  // Listen for progress events
  onMount(async () => {
    console.log('[PROGRESS] Setting up event listener for load_progress');
    const unlisten = await listen<LoadProgressEvent>('load_progress', (event) => {
      console.log('[PROGRESS] Received event:', event.payload);
      console.log('[PROGRESS] Layer:', event.payload.layer);
      console.log('[PROGRESS] Loaded:', event.payload.loaded);
      console.log('[PROGRESS] Total:', event.payload.total);
      console.log('[PROGRESS] Progress %:', event.payload.progress);
      setLoadProgress(event.payload);
    });
    
    // Load instrument info on mount
    loadInstrumentInfo();
    
    return () => {
      console.log('[PROGRESS] Cleaning up event listener');
      unlisten();
    };
  });

  // Get appropriate velocity for the selected layer
  const getVelocityForLayer = (layer: string): number => {
    switch (layer) {
      case "PP": return 20;  // Very soft (1-40 range)
      case "MP": return 54;  // Medium soft (41-67 range) 
      case "MF": return 76;  // Medium hard (68-84 range)
      case "FF": return 106; // Very hard (85-127 range)
      default: return 100;
    }
  };

  const noteOn = async (midi: number) => {
    if (activeNotes().has(midi)) return;
    setActiveNotes(prev => new Set(prev).add(midi));

    console.log(`[FRONTEND] Playing note ${midi}, layer: ${selectedLayer()}`);

    try {
      await invoke("play_midi_note", {
        midiNum: midi,
        velocity: getVelocityForLayer(selectedLayer()),
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
    selectedLayer,
    setSelectedLayer,
    hoveredLayer,
    setHoveredLayer,
    isLayerLoading,
    loadProgress,
    availableLayers,
    noteOn,
    noteOff,
    getKeyToMidi
  };
}
