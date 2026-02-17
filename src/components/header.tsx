import VelocitySelector from "./velocity";
import VolumeControl from "./volume";

interface HeaderProps {
  selectedLayer: string;
  onLayerChange: (layer: string) => void;
  layers: string[];
}

export default function Header(props: HeaderProps) {
  const { selectedLayer, onLayerChange, layers } = props;
  
  return (
    <div class="shrink-0 p-4 flex items-center justify-between border-b border-zinc-900 bg-black/40 backdrop-blur-sm">
      <div class="flex items-center gap-6">
        <h1 class="text-xl font-black italic text-emerald-500 tracking-tighter uppercase leading-none">Raku Grand</h1>
        <VelocitySelector
          selectedLayer={selectedLayer}
          onLayerChange={onLayerChange}
          layers={layers}
        />
      </div>
      <VolumeControl />
    </div>
  );
}
