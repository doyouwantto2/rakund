interface WhiteKeyProps {
  midi: number;
  active: () => boolean;
  onMouseDown: () => void;
  onMouseUp: () => void;
  onMouseLeave: () => void;
}

export default function WhiteKey(props: WhiteKeyProps) {
  const { midi, active, onMouseDown, onMouseUp, onMouseLeave } = props;

  return (
    <button
      onMouseDown={onMouseDown}
      onMouseUp={onMouseUp}
      onMouseLeave={onMouseLeave}
      class={`h-full w-[1.923%] shrink-0 bg-zinc-200 border-x border-zinc-400 rounded-b-sm flex items-end justify-center pb-2 text-[7px] font-black text-zinc-500 transition-all duration-150
        ${active() ? "bg-emerald-400 border-emerald-500 translate-y-2 shadow-lg" : "hover:bg-white"}`}
    >
      {midi % 12 === 0 ? "C" + (Math.floor(midi / 12) - 1) : ""}
    </button>
  );
}
