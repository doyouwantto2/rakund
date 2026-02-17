import { For } from "solid-js";

interface VelocitySelectorProps {
  selectedLayer: () => string;
  onLayerChange: (layer: string) => void;
  layers: string[];
}

export default function VelocitySelector(props: VelocitySelectorProps) {
  const { selectedLayer, onLayerChange, layers } = props;
  
  return (
    <div class="flex items-center gap-1 bg-zinc-900 p-1 rounded-lg border border-zinc-800">
      <For each={layers}>{(l) => (
        <button
          onClick={() => onLayerChange(l)}
          class={`px-3 py-1 rounded text-[9px] font-bold transition-all ${selectedLayer() === l ? "bg-emerald-500 text-black shadow-md" : "text-zinc-500 hover:text-zinc-200"
            }`}
        >
          {l}
        </button>
      )}</For>
    </div>
  );
}