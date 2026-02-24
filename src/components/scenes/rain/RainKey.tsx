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

  // Dynamic text color based on velocity
  // Low velocity = darker text for contrast, high velocity = lighter text
  let textR, textG, textB;
  if (velocityRatio < 0.5) {
    // Light background: dark text
    textR = Math.floor(255 - (velocityRatio * 2 * 200)); // 255 to 55
    textG = Math.floor(255 - (velocityRatio * 2 * 200)); // 255 to 55  
    textB = Math.floor(255 - (velocityRatio * 2 * 200)); // 255 to 55
  } else {
    // Dark background: light text
    textR = 255;
    textG = 255;
    textB = 255;
  }

  const bgColor = () => `rgba(${r}, ${g}, ${b}, 0.95)`;
  const borderColor = () => "rgba(255, 255, 255, 0.8)";
  const shadowColor = () => `rgba(${Math.floor(r * 0.3)}, ${Math.floor(g * 0.3)}, ${Math.floor(b * 0.3)}, 0.8)`;

  return (
    <div
      class="absolute overflow-hidden rounded-md"
      style={{
        left: `${props.x}px`,
        top: `${props.y}px`,
        width: `${props.width}px`,
        height: `${Math.max(props.height, 4)}px`,
        background: bgColor(),
        border: `1px solid ${borderColor()}`,
        "box-shadow": `inset 0 1px 0 rgba(255, 255, 255, 0.3), 0 1px 3px ${shadowColor()}`,
      }}
    >
      <span
        class="font-black tabular-nums whitespace-nowrap absolute left-0 right-0"
        style={{
          "font-size": "0.8rem",
          color: "rgba(255, 255, 255, 0.5)",
          "line-height": "1",
          "text-shadow": `0 1px 2px rgba(0, 0, 0, 0.3)`,
          "letter-spacing": "0.5px",
          "text-align": "center",
          "bottom": "1rem",
        }}
      >
        {props.note.velocity}
      </span>
    </div>
  );
}
