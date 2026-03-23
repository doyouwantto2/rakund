import { For, createMemo, onMount, onCleanup, createSignal } from "solid-js";
import type { Accessor } from "solid-js";
import type { MidiNoteMs, SessionMode } from "@/hooks/useBuffer";
import { getKeyLayoutPx, type KeyLayoutPx } from "@/utils/pianoLayout";
import RainKey from "./RainKey";

// ── Constants ─────────────────────────────────────────────────────────────────

const LOOKAHEAD_MS = 3000;
// Generous upper bound for a single note's duration.
// Notes that started this long ago but are still tall enough to be visible
// are kept in the window.
const MAX_NOTE_DURATION_MS = 8000;

// ── Types ─────────────────────────────────────────────────────────────────────

interface PreparedNote {
  // Stable identity — never recreated for the same note
  key: string;
  note: MidiNoteMs;
  layout: KeyLayoutPx;
  isBlack: boolean;
}

// ── Props ─────────────────────────────────────────────────────────────────────

interface RainLayoutProps {
  allNotes: Accessor<MidiNoteMs[]>;
  currentTime: Accessor<number>;
  sessionMode: Accessor<SessionMode>;
}

// ── Binary search helpers ─────────────────────────────────────────────────────

function lowerBound(notes: PreparedNote[], target: number): number {
  let lo = 0, hi = notes.length;
  while (lo < hi) {
    const mid = (lo + hi) >>> 1;
    if (notes[mid].note.start_ms < target) lo = mid + 1;
    else hi = mid;
  }
  return lo;
}

function upperBound(notes: PreparedNote[], target: number): number {
  let lo = 0, hi = notes.length;
  while (lo < hi) {
    const mid = (lo + hi) >>> 1;
    if (notes[mid].note.start_ms <= target) lo = mid + 1;
    else hi = mid;
  }
  return lo;
}

// ── Component ─────────────────────────────────────────────────────────────────

export default function RainLayout(props: RainLayoutProps) {
  let containerRef: HTMLDivElement | undefined;

  const [containerWidth, setContainerWidth] = createSignal(0);
  const [containerHeight, setContainerHeight] = createSignal(0);

  onMount(() => {
    if (!containerRef) return;
    const measure = () => {
      setContainerWidth(containerRef!.clientWidth);
      setContainerHeight(containerRef!.clientHeight);
    };
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(containerRef);
    onCleanup(() => observer.disconnect());
  });

  const speed = createMemo(() => containerHeight() / LOOKAHEAD_MS);

  // ── preparedNotes — runs ONCE per song load (or resize) ──────────────────
  //
  // Sorted by start_ms and with pixel layouts pre-computed.
  // Object references here are STABLE — the same PreparedNote object is reused
  // across every frame so <For> can track identity correctly.

  const preparedNotes = createMemo<PreparedNote[]>(() => {
    const w = containerWidth();
    const notes = props.allNotes();
    if (w === 0 || notes.length === 0) return [];

    const result: PreparedNote[] = [];
    for (let i = 0; i < notes.length; i++) {
      const note = notes[i];
      const layout = getKeyLayoutPx(note.midi, w);
      if (!layout) continue;
      result.push({
        key: `${note.midi}-${note.start_ms}`,
        note,
        layout,
        isBlack: layout.type === "black",
      });
    }

    result.sort((a, b) => a.note.start_ms - b.note.start_ms);
    return result;
  });

  // ── windowNotes — runs per frame but returns STABLE object references ─────
  //
  // Binary-searches the sorted preparedNotes to find the slice in the visible
  // window. Crucially it returns the PreparedNote objects themselves (not new
  // wrappers), so <For> can track them by reference and never remounts a
  // RainKey that is still on screen.
  //
  // Per-frame position (y, height) is NOT computed here — it is computed
  // inside RainKey via the accessors passed as props, meaning only the style
  // attribute of the DOM node is touched each frame, not the component tree.

  const windowNotes = createMemo(() => {
    const t = props.currentTime();
    const h = containerHeight();
    const spd = speed();
    const prepared = preparedNotes();
    if (h === 0 || spd === 0 || prepared.length === 0) return [];

    const windowStart = t - MAX_NOTE_DURATION_MS;
    const windowEnd = t + LOOKAHEAD_MS;

    const from = lowerBound(prepared, windowStart);
    const to = upperBound(prepared, windowEnd);

    // Quick pass: only keep notes that are actually visible right now.
    // We still return the SAME PreparedNote references, not new objects.
    const result: PreparedNote[] = [];
    for (let i = from; i < to; i++) {
      const pn = prepared[i];
      const noteHeight = Math.max(pn.note.duration_ms * spd, 4);
      const noteBottom = (t - pn.note.start_ms + LOOKAHEAD_MS) * spd;
      const noteTop = noteBottom - noteHeight;
      if (noteBottom < 0 || noteTop > h) continue;
      result.push(pn);
    }
    return result;
  });

  return (
    <div
      ref={containerRef}
      class="w-full h-full relative overflow-hidden bg-zinc-950"
    >
      {/*
        <For> tracks items by reference (===).
        Because windowNotes returns PreparedNote objects from preparedNotes
        (which are stable), <For> will:
          • mount a new <RainKey> only when a note first enters the window
          • keep it mounted and let it update its own position reactively
          • unmount it only when it leaves the window
        No more destroy-and-recreate every frame.
      */}
      <For each={windowNotes()} fallback={null}>
        {(pn) => {
          // These accessors are closures over `pn` (stable) and reactive signals.
          // They are called inside RainKey's JSX, so only the style string
          // of that one DOM node is updated each frame — no VDOM diffing.
          const spd = speed;
          const h = containerHeight;
          const t = props.currentTime;

          const y = () => {
            const noteHeight = Math.max(pn.note.duration_ms * spd(), 4);
            const noteBottom = (t() - pn.note.start_ms + LOOKAHEAD_MS) * spd();
            return noteBottom - noteHeight;
          };

          const height = () => Math.max(pn.note.duration_ms * spd(), 4);

          return (
            <RainKey
              note={pn.note}
              x={pn.layout.x}
              width={pn.layout.width}
              isBlack={pn.isBlack}
              y={y}
              height={height}
            />
          );
        }}
      </For>
    </div>
  );
}
