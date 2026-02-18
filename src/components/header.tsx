import VelocitySelector from "./velocity";
import VolumeControl from "./volume";

interface HeaderProps {
  selectedLayer: () => string;
  onLayerChange: (layer: string) => void;
  onLayerHover?: (layer: string | null) => void;
  layers: () => string[];
}

export default function Header(props: HeaderProps) {
  const { selectedLayer, onLayerChange, onLayerHover, layers } = props;

  return (
    <div class="shrink-0 p-6 flex items-center justify-between border-b border-zinc-900 bg-gradient-to-r from-zinc-950 via-zinc-900 to-zinc-950 backdrop-blur-sm">
      <div class="flex items-center gap-8">
        {/* Logo */}
        <div class="flex items-center gap-4">
          <div class="relative">
            <div class="w-12 h-12 bg-gradient-to-br from-emerald-500 to-emerald-600 rounded-xl flex items-center justify-center shadow-lg shadow-emerald-500/20">
              <div class="text-white font-black text-xl">R</div>
            </div>
            <div class="absolute -top-1 -right-1 w-3 h-3 bg-emerald-400 rounded-full animate-pulse"></div>
          </div>
          <div>
            <h1 class="text-2xl font-black italic text-emerald-500 tracking-tighter uppercase leading-none">Raku</h1>
          </div>
        </div>

        {/* Velocity Selector */}
        <VelocitySelector
          selectedLayer={selectedLayer}
          onLayerChange={onLayerChange}
          onLayerHover={onLayerHover}
          layers={layers}
          instrumentName={() => "instrument"}
        />
      </div>

      <VolumeControl />
    </div>
  );
}
