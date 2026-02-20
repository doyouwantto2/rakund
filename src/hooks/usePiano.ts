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

export interface Contribution {
  authors: string[];
  published_date: string;
  licenses: string[];
}

export interface InstrumentInfo {
  name: string;
  folder: string;
  layers: string[];
  format: string;
  settings?: [string, string][];
  contribution?: Contribution;
}

export function usePiano() {
  const [activeNotes, setActiveNotes] = createSignal(new Set<number>());
  const [volume, setVolume] = createSignal(1.0);
  const [availableLayers, setAvailableLayers] = createSignal<string[]>([]);
  const [availableInstruments, setAvailableInstruments] = createSignal<InstrumentInfo[]>([]);
  const [currentInstrument, setCurrentInstrument] = createSignal<InstrumentInfo | null>(null);
  const [isLoading, setIsLoading] = createSignal(false);
  const [loadProgress, setLoadProgress] = createSignal<number | null>(null);

  // The single source of truth for "what is currently loading or loaded".
  // Set immediately when a load starts (before any await), cleared when done.
  // This is what the guard checks — not currentInstrument, which lags behind.
  const [activeFolder, setActiveFolder] = createSignal<string | null>(null);

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
    const defaultIdx = info.layers.findIndex(l => l.toUpperCase() === "MP");
    const idx = defaultIdx >= 0 ? defaultIdx : 0;
    setLeftLayerIdx(idx);
    setRightLayerIdx(idx);
    // Mark this folder as the active one so the guard knows
    if (info.folder) setActiveFolder(info.folder);
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

  const velocityForLayer = (layer: string): number => {
    if (!layer) return 54;
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

  // ── Instrument management ─────────────────────────────────────────────────────

  const loadAvailableInstruments = async () => {
    try {
      const instruments = await invoke<InstrumentInfo[]>("get_available_instruments");
      setAvailableInstruments(instruments);
      return instruments;
    } catch (e) {
      console.error("[INSTRUMENTS] scan error:", e);
      return [] as InstrumentInfo[];
    }
  };

  const selectInstrument = async (folder: string) => {
    // PRIMARY GUARD: if this folder is already active (loading or loaded), do nothing
    if (activeFolder() === folder) {
      console.log("[INSTRUMENTS] already active:", folder);
      return;
    }

    // SECONDARY GUARD: if another instrument is currently loading, do nothing
    if (isLoading() && activeFolder() !== folder) {
      console.log("[INSTRUMENTS] Another instrument is loading, blocking selection:", folder);
      return;
    }

    console.log("[INSTRUMENTS] loading:", folder);
    setActiveFolder(folder);
    setIsLoading(true);
    setLoadProgress(0);

    // Show name immediately from available list while loading
    const placeholder = availableInstruments().find(i => i.folder === folder);
    if (placeholder && !currentInstrument()) {
      setAvailableLayers(placeholder.layers);
      const defaultIdx = placeholder.layers.findIndex(l => l.toUpperCase() === "MP");
      setLeftLayerIdx(defaultIdx >= 0 ? defaultIdx : 0);
      setRightLayerIdx(defaultIdx >= 0 ? defaultIdx : 0);
    }

    try {
      // Backend returns placeholder info immediately and starts background loading
      const info = await invoke<InstrumentInfo>("load_instrument", { folder });
      console.log("[INSTRUMENTS] Backend started background load for:", info.name);
      
      // Apply placeholder info immediately (real info will come via progress events)
      applyInstrument(info);
      
      // Keep loading state true - background task will emit "done" event when complete
      console.log("[INSTRUMENTS] Waiting for background load to complete...");
      
    } catch (e) {
      console.error("[INSTRUMENTS] load error:", e);
      setActiveFolder(null);
      setIsLoading(false);
      setLoadProgress(null);
    }
    // Note: No finally block - loading state is cleared only when background load completes via progress event
  };

  // ── Audio ─────────────────────────────────────────────────────────────────────

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

  // ── Startup ───────────────────────────────────────────────────────────────────

  onMount(async () => {
    // 1. Scan instrument list (reads JSON headers only, fast)
    const instruments = await loadAvailableInstruments();

    // 2. Read last_instrument from state to know what will be background-loading.
    //    Set activeFolder immediately so any click on that instrument is blocked.
    try {
      const appState = await invoke<{ last_instrument: string | null }>("get_app_state");
      const lastFolder = appState.last_instrument;

      if (lastFolder) {
        // Pre-populate layers from the scanned list for immediate UI feedback
        const placeholder = instruments.find(i => i.folder === lastFolder);
        if (placeholder) {
          setAvailableLayers(placeholder.layers);
          const defaultIdx = placeholder.layers.findIndex(l => l.toUpperCase() === "MP");
          setLeftLayerIdx(defaultIdx >= 0 ? defaultIdx : 0);
          setRightLayerIdx(defaultIdx >= 0 ? defaultIdx : 0);
        }

        // Mark as active so the guard blocks duplicate loads
        setActiveFolder(lastFolder);
        setIsLoading(true);
      }
    } catch (e) {
      console.error("[INIT] get_app_state error:", e);
    }

    // 3. Check if backend already finished loading before we got here (fast restart)
    try {
      const info = await invoke<InstrumentInfo | null>("get_instrument_info");
      if (info && info.folder) {
        applyInstrument(info);
        setIsLoading(false);
        setLoadProgress(null);
      }
    } catch (e) {
      console.error("[INIT] get_instrument_info error:", e);
    }

    // 4. Listen for background preload progress events
    const unlisten = await listen<{
      progress: number;
      status: "loading" | "done" | "error";
      message?: string;
    }>("load_progress", async (e) => {
      const { progress, status } = e.payload;

      if (status === "loading") {
        setIsLoading(true);
        setLoadProgress(progress);
      } else if (status === "done") {
        setLoadProgress(100);
        try {
          const info = await invoke<InstrumentInfo | null>("get_instrument_info");
          if (info && info.folder) applyInstrument(info);
        } catch { /* ignore */ }
        setTimeout(() => {
          setIsLoading(false);
          setLoadProgress(null);
        }, 400);
      } else if (status === "error") {
        console.error("[PRELOAD] failed:", e.payload.message);
        setIsLoading(false);
        setLoadProgress(null);
        setActiveFolder(null);
      }
    });

    // ── Keyboard ──────────────────────────────────────────────────────────────

    const heldKeys = new Set<string>();
    const keyToMidi = new Map<string, number>();

    const recomputeModifiers = () => {
      const lm: Modifier = heldModifiers.altLeft ? 'flat' : heldModifiers.shiftLeft ? 'sharp' : null;
      const rm: Modifier = heldModifiers.altRight ? 'flat' : heldModifiers.shiftRight ? 'sharp' : null;
      setLeftModifier(lm);
      setRightModifier(rm);
    };

    const normalizeKey = (k: string) => {
      if (k === '?') return '/';
      if (k === '<') return ',';
      if (k === '>') return '.';
      if (k === ':') return ';';
      return k.toLowerCase();
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
        if (heldModifiers.shiftLeft || heldModifiers.altLeft) cycleLayer('left', dir);
        if (heldModifiers.shiftRight || heldModifiers.altRight) cycleLayer('right', dir);
        return;
      }

      const key = normalizeKey(e.key);

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

      const key = normalizeKey(e.key);
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
    velocityForLayer,
    volume, setVolume,
    availableInstruments, currentInstrument,
    activeFolder,
    selectInstrument, isLoading, loadProgress,
    leftSection, rightSection,
    leftModifier, rightModifier,
  };
}
