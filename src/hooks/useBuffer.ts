import { createSignal, onMount, onCleanup } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
// pianoEvent will be imported here when perform mode scoring is implemented

// ── Types ─────────────────────────────────────────────────────────────────────

export interface SongInfo {
  file_name: string;
  file_path: string;
  display_name: string;
}

export interface MidiNoteMs {
  midi: number;
  velocity: number;
  start_ms: number;
  duration_ms: number;
  channel: number;
}

export interface MidiSessionInfo {
  total_duration_ms: number;
  tempo_bpm: number;
  note_count: number;
  file_path: string;
}

export type SessionMode = "perform" | "instruct" | null;
export type SessionStatus =
  | "idle"
  | "loading"
  | "ready"
  | "playing"
  | "paused"
  | "finished";

// ── Hook ──────────────────────────────────────────────────────────────────────

export function useBuffer() {
  // ── Song list (from scan) ──────────────────────────────────────────────────
  const [availableSongs, setAvailableSongs] = createSignal<SongInfo[]>([]);
  const [activeSong, setActiveSong] = createSignal<SongInfo | null>(null);

  // ── Session data — write-once after load, never mutated during playback ────
  const [allNotes, setAllNotes] = createSignal<MidiNoteMs[]>([]);
  const [sessionInfo, setSessionInfo] = createSignal<MidiSessionInfo | null>(
    null,
  );

  // ── Loading state ──────────────────────────────────────────────────────────
  const [isLoading, setIsLoading] = createSignal(false);
  const [sessionStatus, setSessionStatus] = createSignal<SessionStatus>("idle");

  // ── Mode — only selectable after loading completes ─────────────────────────
  const [sessionMode, setSessionMode] = createSignal<SessionMode>(null);

  // ── Playback clock ─────────────────────────────────────────────────────────
  const [currentTime, setCurrentTime] = createSignal(0);
  let sessionStartMs = 0;
  let rafHandle: number | null = null;

  // ── Score (perform mode only) ──────────────────────────────────────────────
  const [score, setScore] = createSignal(0);

  // ── Helpers ───────────────────────────────────────────────────────────────

  const isReady = () =>
    sessionStatus() === "ready" ||
    sessionStatus() === "playing" ||
    sessionStatus() === "paused";

  // ── Scan ──────────────────────────────────────────────────────────────────

  const scanSongs = async () => {
    try {
      const songs = await invoke<SongInfo[]>("scan_songs");
      setAvailableSongs(songs);
    } catch (e) {
      console.error("[SONGS] scan error:", e);
    }
  };

  // ── Load session into RAM ─────────────────────────────────────────────────
  // Mirrors selectInstrument() in usePiano — one call loads everything into RAM.
  // After this completes, allNotes is populated and mode buttons are shown.

  const loadSession = async (song: SongInfo) => {
    // Guard: don't reload the same song, don't start a new load mid-load
    if (isLoading()) return;
    if (activeSong()?.file_path === song.file_path && isReady()) return;

    setActiveSong(song);
    setIsLoading(true);
    setSessionStatus("loading");
    setSessionMode(null);
    setAllNotes([]);
    setSessionInfo(null);
    resetPlayback();

    try {
      // Step 1: parse MIDI + convert ticks→ms, store in backend RAM
      const info = await invoke<MidiSessionInfo>("load_midi_session", {
        filePath: song.file_path,
      });
      setSessionInfo(info);

      // Step 2: fetch the full note list — this is the only time we call the backend
      // for note data. After this, allNotes is the source of truth.
      const notes = await invoke<MidiNoteMs[]>("get_session_notes");
      setAllNotes(notes);

      setSessionStatus("ready");
      console.log(
        `[BUFFER] Session ready — ${notes.length} notes, ${info.tempo_bpm.toFixed(1)} BPM`,
      );
    } catch (e) {
      console.error("[BUFFER] load error:", e);
      setActiveSong(null);
      setSessionStatus("idle");
      setAllNotes([]);
      setSessionInfo(null);
    } finally {
      setIsLoading(false);
    }
  };

  // ── Mode selection ────────────────────────────────────────────────────────
  // Only callable when session is ready. Starts the playback clock.

  const selectMode = (mode: "perform" | "instruct") => {
    if (sessionStatus() !== "ready") return;
    setSessionMode(mode);
    startPlayback();
  };

  // ── Playback clock ────────────────────────────────────────────────────────

  const startPlayback = () => {
    sessionStartMs = performance.now();
    setCurrentTime(0);
    setSessionStatus("playing");
    tick();
  };

  const pausePlayback = () => {
    if (sessionStatus() !== "playing") return;
    stopTick();
    setSessionStatus("paused");
  };

  const resumePlayback = () => {
    if (sessionStatus() !== "paused") return;
    // Adjust start time so currentTime continues from where it left off
    sessionStartMs = performance.now() - currentTime();
    setSessionStatus("playing");
    tick();
  };

  const stopPlayback = () => {
    stopTick();
    setCurrentTime(0);
    setScore(0);
    sessionStartMs = 0;
    setAllNotes([]);
    setSessionInfo(null);
    setSessionStatus("idle");
    setSessionMode(null);
  };

  const resetPlayback = () => {
    stopTick();
    setCurrentTime(0);
    setScore(0);
    sessionStartMs = 0;
  };

  const tick = () => {
    rafHandle = requestAnimationFrame(() => {
      if (sessionStatus() !== "playing") return;

      const now = performance.now() - sessionStartMs;
      setCurrentTime(now);

      const info = sessionInfo();
      if (info && now >= info.total_duration_ms) {
        setSessionStatus("finished");
        stopTick();
        return;
      }

      tick();
    });
  };

  const stopTick = () => {
    if (rafHandle !== null) {
      cancelAnimationFrame(rafHandle);
      rafHandle = null;
    }
  };

  // ── Clear session ─────────────────────────────────────────────────────────

  const clearSession = async () => {
    stopPlayback();
    setActiveSong(null);
    setAllNotes([]);
    setSessionInfo(null);
    setSessionStatus("idle");
    setSessionMode(null);
    try {
      await invoke("clear_session");
    } catch (e) {
      console.error("[BUFFER] clear error:", e);
    }
  };

  // ── Startup ───────────────────────────────────────────────────────────────

  onMount(() => {
    scanSongs();
  });

  onCleanup(() => {
    stopTick();
  });

  // ── Public API ────────────────────────────────────────────────────────────

  return {
    // Song list
    availableSongs,
    activeSong,
    loadSession,
    scanSongs,

    // Session data — passed down to SceneDispatcher / RainLayout
    allNotes,
    sessionInfo,

    // Status
    isLoading,
    sessionStatus,
    isReady,

    // Mode — passed down to BufferContainer for the mode buttons
    sessionMode,
    selectMode,

    // Playback controls
    currentTime,
    startPlayback,
    pausePlayback,
    resumePlayback,
    stopPlayback,
    clearSession,

    // Score (perform mode)
    score,
    setScore,
  };
}
