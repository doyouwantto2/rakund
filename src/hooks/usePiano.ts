import { createSignal, onMount, createEffect } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  getMidiForKey,
  isLeftSectionKey,
  isRightSectionKey,
  isPianoKey,
  LEFT_SECTION_KEYS,
  RIGHT_SECTION_KEYS,
  type SectionNum,
} from "../utils/keyMapping";

interface LoadProgressEvent {
  layer: string;
  loaded: number;
  total: number;
  progress: number;
}

export function usePiano() {
  const [activeNotes, setActiveNotes] = createSignal(new Set<number>());
  const [selectedLayer, setSelectedLayer] = createSignal("MP");
  const [hoveredLayer, setHoveredLayer] = createSignal<string | null>(null);
  const [isLayerLoading, setIsLayerLoading] = createSignal(false);
  const [availableLayers, setAvailableLayers] = createSignal<string[]>(["PP", "MP", "MF", "FF"]);
  const [loadProgress, setLoadProgress] = createSignal<{
    loaded: number; total: number; progress: number;
  } | null>(null);

  // Single active section per hand (null = no section selected)
  const [leftSection, setLeftSection] = createSignal<SectionNum | null>(null);
  const [rightSection, setRightSection] = createSignal<SectionNum | null>(null);

  // Modifier keys tracked independently (L/R)
  const [leftAlt, setLeftAlt] = createSignal(false);
  const [rightAlt, setRightAlt] = createSignal(false);
  const [leftShift, setLeftShift] = createSignal(false);
  const [rightShift, setRightShift] = createSignal(false);

  const getSemitoneOffset = () => {
    const up = (leftShift() ? 1 : 0) + (rightShift() ? 1 : 0);
    const down = (leftAlt() ? 1 : 0) + (rightAlt() ? 1 : 0);
    return up - down;
  };

  const getVelocityForLayer = (layer: string): number => {
    switch (layer) {
      case "PP": return 20;
      case "MP": return 54;
      case "MF": return 76;
      case "FF": return 106;
      default: return 80;
    }
  };

  const noteOn = async (midi: number) => {
    if (activeNotes().has(midi)) return;
    setActiveNotes(prev => new Set(prev).add(midi));
    try {
      await invoke("play_midi_note", {
        midiNum: midi,
        velocity: getVelocityForLayer(selectedLayer()),
        layer: selectedLayer(),
      });
    } catch (e) { console.error("[PIANO] play error:", e); }
  };

  const noteOff = async (midi: number) => {
    setActiveNotes(prev => { const n = new Set(prev); n.delete(midi); return n; });
    try {
      await invoke("stop_midi_note", { midiNum: midi });
    } catch (e) { console.error("[PIANO] stop error:", e); }
  };

  const loadLayer = async (layer: string) => {
    setIsLayerLoading(true);
    try { await invoke("load_instrument_layer", { layer }); }
    catch (e) { console.error("[LAYER] error:", e); }
    finally { setIsLayerLoading(false); setLoadProgress(null); }
  };

  createEffect(() => { loadLayer(selectedLayer()); });

  onMount(async () => {
    try {
      const info = await invoke<{ name: string; layers: string[] }>("get_instrument_info");
      setAvailableLayers(info.layers);
    } catch (e) { console.error("[INSTRUMENT] error:", e); }

    const unlisten = await listen<LoadProgressEvent>("load_progress", (e) => {
      setLoadProgress(e.payload);
    });

    const heldKeys = new Set<string>();
    const keyToMidis = new Map<string, number[]>();

    const handleKeyDown = (e: KeyboardEvent) => {
      // Modifier keys
      if (e.code === "AltLeft") { setLeftAlt(true); e.preventDefault(); return; }
      if (e.code === "AltRight") { setRightAlt(true); e.preventDefault(); return; }
      if (e.code === "ShiftLeft") { setLeftShift(true); return; }
      if (e.code === "ShiftRight") { setRightShift(true); return; }

      if (e.repeat) return;
      const key = e.key === ';' ? ';' : e.key.toLowerCase();

      // Section SWITCH — sets the section, replaces any previous
      if (isLeftSectionKey(key)) {
        setLeftSection(LEFT_SECTION_KEYS[key] as SectionNum);
        return;
      }
      if (isRightSectionKey(key)) {
        setRightSection(RIGHT_SECTION_KEYS[key] as SectionNum);
        return;
      }
      if (key === 'escape') {
        setLeftSection(null);
        setRightSection(null);
        return;
      }

      if (!isPianoKey(key)) return;
      if (heldKeys.has(key)) return;
      heldKeys.add(key);

      const offset = getSemitoneOffset();

      const leftMidis = getMidiForKey(key, leftSection(), 'left', offset);
      const rightMidis = getMidiForKey(key, rightSection(), 'right', offset);
      const midis = [...new Set([...leftMidis, ...rightMidis])];

      if (midis.length > 0) {
        e.preventDefault();
        keyToMidis.set(key, midis);
        midis.forEach(noteOn);
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      if (e.code === "AltLeft") { setLeftAlt(false); return; }
      if (e.code === "AltRight") { setRightAlt(false); return; }
      if (e.code === "ShiftLeft") { setLeftShift(false); return; }
      if (e.code === "ShiftRight") { setRightShift(false); return; }

      const key = e.key === ';' ? ';' : e.key.toLowerCase();
      heldKeys.delete(key);
      const midis = keyToMidis.get(key);
      if (midis) { midis.forEach(noteOff); keyToMidis.delete(key); }
    };

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);

    return () => {
      unlisten();
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  });

  return {
    activeNotes, selectedLayer, setSelectedLayer,
    hoveredLayer, setHoveredLayer,
    isLayerLoading, loadProgress, availableLayers,
    noteOn, noteOff,
    leftSection, rightSection,
    leftAlt, rightAlt, leftShift, rightShift,
    getSemitoneOffset,
  };
}
