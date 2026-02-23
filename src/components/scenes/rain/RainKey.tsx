import type { MidiNoteMs } from "@/hooks/useBuffer";

interface RainKeyProps {
  note: MidiNoteMs;
  x: number;
  width: number;
  y: number;
  height: number;
  isBlack: boolean;
}

export default function RainKey(props: RainKeyProps) {
  const bgColor = () =>
    props.isBlack ? "rgba(194, 90, 0, 0.95)" : "rgba(234, 120, 20, 0.95)";

  const borderColor = () =>
    props.isBlack ? "rgba(160, 70, 0, 1)" : "rgba(255, 150, 50, 1)";

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
        "align-items": "flex-end",
        "justify-content": "center",
        "padding-bottom": "2px",
      }}
    >
      <span
        style={{
          "font-size": "6px",
          "font-weight": "700",
          color: "rgba(255, 255, 255, 0.9)",
          "line-height": "1",
          "font-variant-numeric": "tabular-nums",
          "white-space": "nowrap",
        }}
      >
        {props.note.velocity}
      </span>
    </div>
  );
}
