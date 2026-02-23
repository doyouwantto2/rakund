import type { SessionStatus, SessionMode } from "@/hooks/useBuffer";

interface BufferContainerProps {
  sessionStatus: () => SessionStatus;
  sessionMode: () => SessionMode;
}

export default function BufferContainer(props: BufferContainerProps) {
  const accentColor = () => {
    switch (props.sessionStatus()) {
      case "playing":
        return props.sessionMode() === "perform"
          ? "bg-blue-500"
          : "bg-emerald-500";
      case "paused":
        return "bg-amber-400";
      case "finished":
        return "bg-zinc-600";
      case "ready":
        return "bg-zinc-700";
      default:
        return "bg-zinc-800";
    }
  };

  return (
    <div class="w-full relative flex-shrink-0" style={{ height: "3px" }}>
      <div
        class={`w-full h-full transition-colors duration-300 ${accentColor()}`}
      />
    </div>
  );
}
