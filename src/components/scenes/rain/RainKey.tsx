import type { MidiNoteMs } from "@/hooks/useBuffer";

interface RainKeyProps {
  note: MidiNoteMs;
  x: number;
  width: number;
  y: number; // top of the rectangle in px
  height: number; // height of the rectangle in px
  isBlack: boolean;
}

export default function RainKey(props: RainKeyProps) {
  // Velocity digits stacked vertically, reversed so they read bottom-up.
  // e.g. 120 → ["0", "2", "1"] displayed top→bottom, read bottom→top = 120
  const digits = () => String(props.note.velocity).split("").reverse();

  const bgColor = () =>
    props.isBlack ? "rgba(120, 160, 255, 0.85)" : "rgba(160, 200, 255, 0.90)";

  const borderColor = () =>
    props.isBlack ? "rgba(100, 140, 240, 1)" : "rgba(180, 210, 255, 1)";

  return (
    <div
      style={{
        position: "absolute",
        left: `${props.x}px`,
        top: `${props.y}px`,
        width: `${props.width}px`,
        height: `${Math.max(props.height, 4)}px`,
        background: bgColor(),
        border: `1px solid ${borderColor()}`,
        "border-radius": "2px 2px 0 0",
        "box-sizing": "border-box",
        overflow: "hidden",
        display: "flex",
        "flex-direction": "column",
        "align-items": "center",
        "justify-content": "flex-end",
        "padding-bottom": "2px",
      }}
    >
      {/* Velocity digits — stacked vertically, reversed (read bottom-up) */}
      <div
        style={{
          display: "flex",
          "flex-direction": "column",
          "align-items": "center",
          "line-height": "1",
          gap: "0px",
        }}
      >
        {digits().map((d) => (
          <span
            style={{
              "font-size": "7px",
              "font-weight": "900",
              color: "rgba(255,255,255,0.85)",
              "line-height": "1.1",
              "font-variant-numeric": "tabular-nums",
            }}
          >
            {d}
          </span>
        ))}
      </div>
    </div>
  );
}
