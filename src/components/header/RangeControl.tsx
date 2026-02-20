interface RangeControlProps {
  value: () => number;
  onChange: (v: number) => void;
  min?: number;
  max?: number;
  step?: number;
  label?: string;
}

export default function RangeControl(props: RangeControlProps) {
  return (
    <div class="flex items-center gap-3 bg-zinc-800 rounded-lg px-4 py-2 border border-zinc-700 flex-shrink-0">
      <span class="text-sm text-zinc-400">{props.label || "Range"}</span>
      <input
        type="range"
        min={props.min || 0}
        max={props.max || 100}
        step={props.step || 1}
        value={props.value()}
        onInput={(e) => props.onChange(Number(e.target.value))}
        class="w-24 accent-zinc-600"
      />
      <span class="text-sm text-zinc-200 font-medium">{Math.round(props.value())}</span>
    </div>
  );
}
