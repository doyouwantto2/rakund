interface VolumeControlProps {
  volume: () => number;
  onVolumeChange: (v: number) => void;
}

export default function VolumeControl(props: VolumeControlProps) {
  return (
    <div class="flex items-center gap-3 bg-zinc-800 rounded-lg px-4 py-2 border border-zinc-700 flex-shrink-0">
      <span class="text-sm text-zinc-400">Volume</span>
      <input
        type="range"
        min="0"
        max="100"
        value={props.volume() * 100}
        onInput={(e) => props.onVolumeChange(Number(e.target.value) / 100)}
        class="w-24 accent-zinc-600"
      />
      <span class="text-sm text-zinc-200 font-medium">{Math.round(props.volume() * 100)}%</span>
    </div>
  );
}
