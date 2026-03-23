import type { Accessor } from "solid-js";
import type { MidiNoteMs } from "@/hooks/useBuffer";

interface RainKeyProps {
  note: MidiNoteMs;
  x: number;           // stable — set once when note enters window
  width: number;       // stable — set once when note enters window
  isBlack: boolean;    // stable
  y: Accessor<number>;       // reactive — changes every frame
  height: Accessor<number>;  // reactive — changes every frame
}

export default function RainKey(props: RainKeyProps) {
  // All colour/style values derived from velocity are computed ONCE on mount
  // since velocity never changes. Only top/height update per frame.
  const velocityRatio = props.note.velocity / 127;

  const baseR = 255, baseG = 121, baseB = 64;
  let r: number, g: number, b: number;

  if (velocityRatio < 0.5) {
    const f = 1 + (0.5 - velocityRatio) * 0.4;
    r = Math.min(255, Math.floor(baseR * f));
    g = Math.min(255, Math.floor(baseG * f));
    b = Math.min(255, Math.floor(baseB * f));
  } else {
    const f = 1 - (velocityRatio - 0.5) * 0.4;
    r = Math.floor(baseR * f);
    g = Math.floor(baseG * f);
    b = Math.floor(baseB * f);
  }

  const textVal = velocityRatio < 0.5
    ? Math.floor(255 - velocityRatio * 2 * 10)
    : 255;

  const bgColor = `rgba(${r}, ${g}, ${b}, 0.95)`;
  const shadowColor = `rgba(${Math.floor(r * 0.3)}, ${Math.floor(g * 0.3)}, ${Math.floor(b * 0.3)}, 0.8)`;
  const textColor = `rgba(${textVal}, ${textVal}, ${textVal}, 0.8)`;

  // Static style string for everything except top/height
  const staticStyle = [
    `left:${props.x}px`,
    `width:${props.width}px`,
    `background:${bgColor}`,
    `border:1px solid rgba(255,255,255,0.8)`,
    `box-shadow:inset 0 1px 0 rgba(255,255,255,0.3),0 1px 3px ${shadowColor}`,
  ].join(";");

  return (
    <div
      class="absolute overflow-hidden rounded-md"
      style={`${staticStyle};top:${props.y()}px;height:${Math.max(props.height(), 4)}px`}
    >
      <span
        class="font-black tabular-nums whitespace-nowrap absolute left-0 right-0"
        style={{
          "font-size": "0.8rem",
          color: textColor,
          "line-height": "1",
          "text-shadow": "0 1px 2px rgba(0,0,0,0.3)",
          "letter-spacing": "0.5px",
          "text-align": "center",
          bottom: "1rem",
        }}
      >
        {props.note.velocity}
      </span>
    </div>
  );
}
