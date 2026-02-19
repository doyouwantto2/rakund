import { createSignal, onMount } from "solid-js";
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
  type Modifier,
} from "../utils/keyMapping";

export interface InstrumentInfo {
  name: string;
  folder: string;
  layers: string[];
  format: string;
}

export function usePiano() {
  const [activeNotes, setActiveNotes] = createSignal(new Set<number>());
  const [volume, setVolume] = createSignal(1.0);
  const [availableLayers, setAvailableLayers] = createSignal<string[]>([]);
  const [availableInstruments, setAvailableInstruments] = createSignal<InstrumentInfo[]>([]);
  const [currentInstrument, setCurrentInstrument] = createSignal<InstrumentInfo | null>(null);
  const [isLoading, setIsLoading] = createSignal(false);
  const [loadProgress, setLoadProgress] = createSignal<number | null>(null);
  const [isCurrentlyLoading, setIsCurrentlyLoading] = createSignal<string | null>(null);

  const [leftSection, setLeftSection] = createSignal<SectionNum | null>(null);
  const [rightSection, setRightSection] = createSignal<SectionNum | null>(null);
  const [leftModifier, setLeftModifier] = createSignal<Modifier>(null);
  const [rightModifier, setRightModifier] = createSignal<Modifier>(null);
  const [leftLayerIdx, setLeftLayerIdx] = createSignal(0);
  const [rightLayerIdx, setRightLayerIdx] = createSignal(0);

  const heldModifiers = { shiftLeft: false, shiftRight: false, altLeft: false, altRight: false };

  // ── Helpers ──────────────────────────────────────────────────────────────────

  const applyInstrument = (info: InstrumentInfo) => {
    setCurrentInstrument(info);
    setAvailableLayers(info.layers);
    const defaultIdx = info.layers.indexOf("MP");
    const idx = defaultIdx >= 0 ? defaultIdx : 0;
    setLeftLayerIdx(idx);
    setRightLayerIdx(idx);
  };

  const layerForHand = (hand: 'left' | 'right'): string => {
    const layers = availableLayers();
    if (layers.length === 0) return "";
    const idx = hand === 'left' ? leftLayerIdx() : rightLayerIdx();
    return layers[Math.min(idx, layers.length - 1)];
  };

  const cycleLayer = (hand: 'left' | 'right', direction: 1 | -1) => {
    const total = availableLayers().length;
    if (total === 0) return;
    if (hand === 'left') {
      setLeftLayerIdx(i => Math.max(0, Math.min(total - 1, i + direction)));
    } else {
      setRightLayerIdx(i => Math.max(0, Math.min(total - 1, i + direction)));
    }
  };

  const loadAvailableInstruments = async () => {
    try {
      const instruments = await invoke<InstrumentInfo[]>("get_available_instruments");
      setAvailableInstruments(instruments);
    } catch (e) {
      console.error("[INSTRUMENTS] scan error:", e);
    }
  };

  const selectInstrument = async (folder: string) => {
    // If the same instrument is currently loaded, do nothing
    if (currentInstrument()?.folder === folder) {
      console.log("[INSTRUMENTS] Same instrument already loaded, skipping:", folder);
      return;
    }

    // If currently loading any instrument, do nothing
    if (isCurrentlyLoading() !== null) {
      console.log("[INSTRUMENTS] Already loading, skipping:", folder);
      return;
    }

    // Set the loading state to prevent multiple clicks
    setIsCurrentlyLoading(folder);
    setIsLoading(true);
    setLoadProgress(0);

    try {
      console.log("[INSTRUMENTS] Loading instrument:", folder);
      const info = await invoke<InstrumentInfo>("load_instrument", { folder });
      applyInstrument(info);
    } catch (e) {
      console.error("[INSTRUMENTS] load error:", e);
    } finally {
      setIsLoading(false);
      setLoadProgress(null);
      setIsCurrentlyLoading(null);
    }
  };

  // ── Audio ────────────────────────────────────────────────────────────────────

  const velocityForLayer = (layer: string): number => {
    const lower = layer.toLowerCase();
    const layers = availableLayers();
    const base =
      lower === "pp" ? 20 :
        lower === "mp" ? 54 :
          lower === "mf" ? 76 :
            lower === "ff" ? 106 :
              (() => {
                const idx = layers.indexOf(layer);
                const total = layers.length;
                return total === 0 ? 54 : Math.round(20 + (idx / Math.max(total - 1, 1)) * 86);
              })();
    return Math.min(127, Math.round(base * volume()));
  };

  const noteOn = async (midi: number, hand: 'left' | 'right') => {
    if (activeNotes().has(midi)) return;
    setActiveNotes(prev => new Set(prev).add(midi));
    const layer = layerForHand(hand);
    try {
      await invoke("play_midi_note", {
        midiNum: midi,
        velocity: velocityForLayer(layer),
        layer,
      });
    } catch (e) { console.error("[PIANO] play error:", e); }
  };

  const noteOff = async (midi: number) => {
    setActiveNotes(prev => { const n = new Set(prev); n.delete(midi); return n; });
    try {
      await invoke("stop_midi_note", { midiNum: midi });
    } catch (e) { console.error("[PIANO] stop error:", e); }
  };

  // ── Startup ──────────────────────────────────────────────────────────────────

  onMount(async () => {
    // Load instrument list immediately — doesn't depend on preload
    await loadAvailableInstruments();

    // Check if backend already has something loaded (e.g. previous session, fast load)
    // Only apply if we're not in the middle of a background preload
    try {
      const info = await invoke<InstrumentInfo | null>("get_instrument_info");
      if (info && !isLoading()) {
        applyInstrument(info);
      }
    } catch (e) {
      console.error("[INIT] get_instrument_info error:", e);
    }

    // Listen for background preload progress events from init.rs
    // This fires when the app starts and backend loads last instrument in background
    const unlisten = await listen<{
      progress: number;
      status: "loading" | "done" | "error";
      loaded?: number;
      total?: number;
      message?: string;
    }>("load_progress", async (e) => {
      const { progress, status } = e.payload;

      if (status === "loading") {
        setIsLoading(true);
        setLoadProgress(progress);
      } else if (status === "done") {
        setLoadProgress(100);
        // Hydrate instrument info now that backend is ready
        try {
          const info = await invoke<InstrumentInfo | null>("get_instrument_info");
          if (info && currentInstrument()?.folder !== info.folder) {
            applyInstrument(info);
          }
        } catch { /* ignore */ }
        // Small delay so the 100% flash is visible
        setTimeout(() => {
          setIsLoading(false);
          setLoadProgress(null);
          setIsCurrentlyLoading(null);
        }, 400);
      } else if (status === "error") {
        console.error("[PRELOAD] Error:", e.payload.message);
        setIsLoading(false);
        setLoadProgress(null);
        setIsCurrentlyLoading(null);
      }
    });

    // ── Keyboard ──────────────────────────────────────────────────────────────

    const heldKeys = new Set<string>();
    const keyToMidi = new Map<string, number>();

    const recomputeModifiers = () => {
      // FIX: If ANY alt or ANY shift is pressed, apply it globally to both hands
      const anyAlt = heldModifiers.altLeft || heldModifiers.altRight;
      const anyShift = heldModifiers.shiftLeft || heldModifiers.shiftRight;

      const globalModifier: Modifier = anyAlt ? 'flat' : anyShift ? 'sharp' : null;

      setLeftModifier(globalModifier);
      setRightModifier(globalModifier);
    };

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.code === "ShiftLeft") { heldModifiers.shiftLeft = true; recomputeModifiers(); return; }
      if (e.code === "ShiftRight") { heldModifiers.shiftRight = true; recomputeModifiers(); return; }
      if (e.code === "AltLeft") { heldModifiers.altLeft = true; recomputeModifiers(); e.preventDefault(); return; }
      if (e.code === "AltRight") { heldModifiers.altRight = true; recomputeModifiers(); e.preventDefault(); return; }
      if (e.repeat) return;

      if (e.code === "Space") {
        e.preventDefault();
        const dir: 1 | -1 = (heldModifiers.shiftLeft || heldModifiers.shiftRight) ? 1 : -1;
        // Space logic still respects exact Left/Right physical keys
        if (heldModifiers.shiftLeft || heldModifiers.altLeft) cycleLayer('left', dir);
        if (heldModifiers.shiftRight || heldModifiers.altRight) cycleLayer('right', dir);
        return;
      }

      const key = e.key === ';' ? ';' : e.key.toLowerCase();

      if (isLeftSectionKey(key)) { setLeftSection(LEFT_SECTION_KEYS[key] as SectionNum); return; }
      if (isRightSectionKey(key)) { setRightSection(RIGHT_SECTION_KEYS[key] as SectionNum); return; }
      if (key === 'escape') { setLeftSection(null); setRightSection(null); return; }
      if (!isPianoKey(key) || heldKeys.has(key)) return;

      heldKeys.add(key);

      const leftMidi = getMidiForKey(key, leftSection(), 'left', leftModifier());
      const rightMidi = getMidiForKey(key, rightSection(), 'right', rightModifier());

      if (leftMidi !== null) {
        e.preventDefault();
        keyToMidi.set(key, leftMidi);
        noteOn(leftMidi, 'left');
      } else if (rightMidi !== null) {
        e.preventDefault();
        keyToMidi.set(key, rightMidi);
        noteOn(rightMidi, 'right');
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      if (e.code === "ShiftLeft") { heldModifiers.shiftLeft = false; recomputeModifiers(); return; }
      if (e.code === "ShiftRight") { heldModifiers.shiftRight = false; recomputeModifiers(); return; }
      if (e.code === "AltLeft") { heldModifiers.altLeft = false; recomputeModifiers(); return; }
      if (e.code === "AltRight") { heldModifiers.altRight = false; recomputeModifiers(); return; }

      const key = e.key === ';' ? ';' : e.key.toLowerCase();
      heldKeys.delete(key);
      const midi = keyToMidi.get(key);
      if (midi !== undefined) { noteOff(midi); keyToMidi.delete(key); }
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
    activeNotes, noteOn, noteOff,
    availableLayers,
    leftLayerIdx, rightLayerIdx,
    layerForHand,
    volume, setVolume,
    availableInstruments, currentInstrument,
    selectInstrument, isLoading, loadProgress,
    leftSection, rightSection,
    leftModifier, rightModifier,
  };
}
