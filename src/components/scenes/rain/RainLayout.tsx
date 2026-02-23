import { For, createMemo, onMount, onCleanup, createSignal } from "solid-js";
// Note: octave boundary lines are rendered in the stage container (Home.tsx),
// not here, so they span both the scene and buffer strip.
import type { Accessor } from "solid-js";
import type { MidiNoteMs, SessionMode } from "@/hooks/useBuffer";
import { getKeyLayoutPx } from "@/utils/pianoLayout";
import RainKey from "./RainKey";

// ── Constants ─────────────────────────────────────────────────────────────────

const LOOKAHEAD_MS = 3000; // notes are visible 3 seconds before they're due

// ── Props ─────────────────────────────────────────────────────────────────────

interface RainLayoutProps {
  allNotes: Accessor<MidiNoteMs[]>;
  currentTime: Accessor<number>;
  sessionMode: Accessor<SessionMode>;
}

// ── Component ─────────────────────────────────────────────────────────────────

export default function RainLayout(props: RainLayoutProps) {
  let containerRef: HTMLDivElement | undefined;

  const [containerWidth, setContainerWidth] = createSignal(0);
  const [containerHeight, setContainerHeight] = createSignal(0);

  // Measure container and watch for resize
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

  // ── Fall speed ───────────────────────────────────────────────────────────────
  // speed (px/ms) = containerHeight / LOOKAHEAD_MS
  // noteBottom reaches containerHeight exactly when currentTime === note.start_ms

  const speed = createMemo(() => containerHeight() / LOOKAHEAD_MS);

  // ── Visible notes ────────────────────────────────────────────────────────────

  const visibleNotes = createMemo(() => {
    const t = props.currentTime();
    const h = containerHeight();
    const w = containerWidth();
    const spd = speed();
    if (w === 0 || h === 0 || spd === 0) return [];

    return props.allNotes().flatMap((note) => {
      const keyLayout = getKeyLayoutPx(note.midi, w);
      if (!keyLayout) return [];

      const noteHeight = Math.max(note.duration_ms * spd, 4);

      // Bottom of rectangle = how far the note has fallen toward the hit line
      const noteBottom = (t - note.start_ms + LOOKAHEAD_MS) * spd;
      const noteTop = noteBottom - noteHeight;

      // Only render if any part of the note is within the visible area
      if (noteBottom < 0 || noteTop > h) return [];

      return [
        {
          note,
          x: keyLayout.x,
          width: keyLayout.width,
          y: noteTop,
          height: noteHeight,
          isBlack: keyLayout.type === "black",
        },
      ];
    });
  });

  return (
    <div
      ref={containerRef}
      class="w-full h-full relative overflow-hidden bg-zinc-950"
    >
      {/* ── Falling notes ── */}
      <For each={visibleNotes()}>
        {(item) => (
          <RainKey
            note={item.note}
            x={item.x}
            width={item.width}
            y={item.y}
            height={item.height}
            isBlack={item.isBlack}
          />
        )}
      </For>
    </div>
  );
}
