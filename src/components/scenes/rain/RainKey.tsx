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
  // Calculate background brightness/darkness based on velocity (0-127)
  // Velocity 64 = base color, lower = lighter, higher = darker
  const velocityRatio = props.note.velocity / 127;
  
  // Base RGB values
  const baseR = 255;
  const baseG = 121;
  const baseB = 64;
  
  let r, g, b;
  
  if (velocityRatio < 0.5) {
    // Low velocity: make it lighter (0-63)
    const lightnessFactor = 1 + ((0.5 - velocityRatio) * 0.4); // Range: 1.0 to 1.2
    r = Math.min(255, Math.floor(baseR * lightnessFactor));
    g = Math.min(255, Math.floor(baseG * lightnessFactor));
    b = Math.min(255, Math.floor(baseB * lightnessFactor));
  } else {
    // High velocity: make it darker (64-127)
    const darknessFactor = 1 - ((velocityRatio - 0.5) * 0.4); // Range: 1.0 to 0.8
    r = Math.floor(baseR * darknessFactor);
    g = Math.floor(baseG * darknessFactor);
    b = Math.floor(baseB * darknessFactor);
  }
  
  const bgColor = () => `rgba(${r}, ${g}, ${b}, 0.9)`;
  const borderColor = () => `rgba(${r}, ${g}, ${b}, 1)`;

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
        "align-items": "center",
        "justify-content": "center",
      }}
    >
      <span
        style={{
          "font-size": "0.6rem",
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
