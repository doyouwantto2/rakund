interface BlackKeyProps {
  active: boolean;
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
      class={`absolute z-20 h-[58%] w-[1.1%] bg-zinc-900 border-x border-b border-black rounded-b-sm transition-all duration-75
        ${active ? "!bg-emerald-700 !border-emerald-500 translate-y-1" : "hover:bg-zinc-800"}`}
      style={style}
    />
  );
}