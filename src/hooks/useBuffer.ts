import { createSignal, onMount, onCleanup } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

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

const VISUAL_LEAD_MS = 2000;
const AUDIO_LEAD_MS = 600;

const BATCH_WINDOW_MS = 8;
const SCHEDULE_WINDOW_MS = 2000;
const SCHEDULER_TICK_MS = 500;

const LATE_TOLERANCE_MS = 50;

export function useBuffer(
  onNoteOn?: (midi: number, velocity: number) => void,
  onNoteOff?: (midi: number) => void,
) {
  const [availableSongs, setAvailableSongs] = createSignal<SongInfo[]>([]);
  const [activeSong, setActiveSong] = createSignal<SongInfo | null>(null);
  const [allNotes, setAllNotes] = createSignal<MidiNoteMs[]>([]);
  const [sessionInfo, setSessionInfo] = createSignal<MidiSessionInfo | null>(
    null,
  );
  const [isLoading, setIsLoading] = createSignal(false);
  const [sessionStatus, setSessionStatus] = createSignal<SessionStatus>("idle");
  const [sessionMode, setSessionMode] = createSignal<SessionMode>(null);
  const [currentTime, setCurrentTime] = createSignal(0);
  const [score, setScore] = createSignal(0);

  let sessionStartMs = 0;
  let rafHandle: number | null = null;
  let pausedAtMs = 0;

  let schedulerHandle: ReturnType<typeof setInterval> | null = null;
  let scheduledUpToMs = 0;

  const activeTimeouts = new Set<number>();

  const scheduleTask = (task: () => void, delayMs: number) => {
    const safeDelay = Math.max(0, delayMs);
    const id = window.setTimeout(() => {
      activeTimeouts.delete(id);
      task();
    }, safeDelay);
    activeTimeouts.add(id);
  };

  const clearAllTasks = () => {
    activeTimeouts.forEach((id) => clearTimeout(id));
    activeTimeouts.clear();
  };

  const isReady = () =>
    sessionStatus() === "ready" ||
    sessionStatus() === "playing" ||
    sessionStatus() === "paused";

  const scanSongs = async () => {
    try {
      setAvailableSongs(await invoke<SongInfo[]>("scan_songs"));
    } catch (e) {
      console.error("[SONGS] scan error:", e);
    }
  };

  const loadSession = async (song: SongInfo) => {
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
      const info = await invoke<MidiSessionInfo>("load_midi_session", {
        filePath: song.file_path,
      });
      setSessionInfo(info);
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

  const selectMode = (mode: "perform" | "instruct") => {
    if (sessionStatus() !== "ready") return;
    setSessionMode(mode);
    startPlayback(mode);
  };

  const scheduleWindow = (
    sorted: MidiNoteMs[],
    fromMs: number,
    layer: string,
  ) => {
    const windowEnd = scheduledUpToMs + SCHEDULE_WINDOW_MS;

    let batchStart = -Infinity;
    let batch: MidiNoteMs[] = [];

    const flushBatch = () => {
      if (batch.length === 0) return;

      const currentBatch = [...batch];
      batch = [];

      const baseStartMs = currentBatch[0].start_ms;
      const noteOnDelay = baseStartMs - fromMs - AUDIO_LEAD_MS;

      if (noteOnDelay < -LATE_TOLERANCE_MS) return;

      scheduleTask(() => {
        invoke("play_notes_batch", {
          notes: currentBatch.map((n) => ({
            midi_num: n.midi,
            velocity: n.velocity,
            layer: layer,
          })),
        }).catch((e) => console.error("[BATCH] IPC error:", e));

        for (const n of currentBatch) {
          onNoteOn?.(n.midi, n.velocity);
        }
      }, noteOnDelay);

      for (const note of currentBatch) {
        const offDelay =
          note.start_ms + note.duration_ms - fromMs - AUDIO_LEAD_MS;
        if (offDelay >= -LATE_TOLERANCE_MS) {
          scheduleTask(() => onNoteOff?.(note.midi), offDelay);
        }
      }
    };

    for (const note of sorted) {
      if (note.start_ms <= scheduledUpToMs) continue;
      if (note.start_ms > windowEnd) break;

      if (note.start_ms - batchStart > BATCH_WINDOW_MS) {
        flushBatch();
        batchStart = note.start_ms;
      }
      batch.push(note);
    }
    flushBatch();

    scheduledUpToMs = windowEnd;
  };

  const startRollingScheduler = (notes: MidiNoteMs[], fromMs: number) => {
    stopRollingScheduler();

    const sorted = [...notes].sort((a, b) => a.start_ms - b.start_ms);
    const layer = "default";

    scheduledUpToMs = fromMs;
    scheduleWindow(sorted, fromMs, layer);

    schedulerHandle = setInterval(() => {
      const animNow = performance.now() - sessionStartMs;
      if (animNow > scheduledUpToMs) scheduledUpToMs = animNow;
      scheduleWindow(sorted, animNow, layer);
    }, SCHEDULER_TICK_MS);
  };

  const stopRollingScheduler = () => {
    if (schedulerHandle !== null) {
      clearInterval(schedulerHandle);
      schedulerHandle = null;
    }
    clearAllTasks();
    scheduledUpToMs = 0;
  };

  const startPlayback = (mode: SessionMode) => {
    sessionStartMs = performance.now() + VISUAL_LEAD_MS;
    pausedAtMs = -VISUAL_LEAD_MS;
    setCurrentTime(-VISUAL_LEAD_MS);
    setSessionStatus("playing");
    if (mode === "instruct") startRollingScheduler(allNotes(), -VISUAL_LEAD_MS);
    tick();
  };

  const pausePlayback = () => {
    if (sessionStatus() !== "playing") return;
    pausedAtMs = currentTime();
    stopRollingScheduler();
    stopTick();
    setSessionStatus("paused");
  };

  const resumePlayback = () => {
    if (sessionStatus() !== "paused") return;
    sessionStartMs = performance.now() - pausedAtMs;
    setSessionStatus("playing");
    if (sessionMode() === "instruct")
      startRollingScheduler(allNotes(), pausedAtMs);
    tick();
  };

  const stopPlayback = () => {
    stopRollingScheduler();
    stopTick();
    setCurrentTime(0);
    setScore(0);
    pausedAtMs = 0;
    sessionStartMs = 0;
    setAllNotes([]);
    setSessionInfo(null);
    setSessionStatus("idle");
    setSessionMode(null);
  };

  const resetPlayback = () => {
    stopRollingScheduler();
    stopTick();
    setCurrentTime(0);
    setScore(0);
    pausedAtMs = 0;
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

  onMount(() => {
    scanSongs();
  });
  onCleanup(() => {
    stopRollingScheduler();
    stopTick();
  });

  return {
    availableSongs,
    activeSong,
    loadSession,
    scanSongs,
    allNotes,
    sessionInfo,
    isLoading,
    sessionStatus,
    isReady,
    sessionMode,
    selectMode,
    currentTime,
    startPlayback,
    pausePlayback,
    resumePlayback,
    stopPlayback,
    clearSession,
    score,
    setScore,
  };
}
