interface BlackKeyProps {
  active: () => boolean;
  onMouseDown: () => void;
  onMouseUp: () => void;
  onMouseLeave: () => void;
  style: string;
}

export default function BlackKey(props: BlackKeyProps) {
  const { active, onMouseDown, onMouseUp, onMouseLeave, style } = props;

  return (
    <button
      onMouseDown={onMouseDown}
      onMouseUp={onMouseUp}
      onMouseLeave={onMouseLeave}
      class={`absolute z-20 h-[58%] w-[1.1%] bg-zinc-900 border-x border-b border-black rounded-b-sm transition-all duration-150
        ${active() ? "bg-emerald-600 border-emerald-400 translate-y-2 shadow-lg" : "hover:bg-zinc-800"}`}
      style={style}
    />
  );
}
