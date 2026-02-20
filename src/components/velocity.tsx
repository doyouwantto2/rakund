import { For, createMemo } from "solid-js";

interface VelocitySelectorProps {
  selectedLayer: () => string;
  onLayerChange: (layer: string) => void;
  onLayerHover?: (layer: string | null) => void;
  layers: () => string[];
  instrumentName: () => string;
}

export default function VelocitySelector(props: VelocitySelectorProps) {
  const { selectedLayer, onLayerChange, onLayerHover, layers, instrumentName } = props;

  const containerClass = createMemo(() => {
    const layerCount = layers().length;
    const isSalamander = instrumentName() === 'salamander';

    if (isSalamander && layerCount > 8) {
      return "flex items-center gap-1 bg-zinc-900/80 p-2 rounded-xl border border-zinc-800/50 backdrop-blur-sm max-w-2xl overflow-x-auto";
    } else if (layerCount > 6) {
      return "flex items-center gap-1 bg-zinc-900/80 p-2 rounded-xl border border-zinc-800/50 backdrop-blur-sm max-w-xl overflow-x-auto";
    } else {
      return "flex items-center gap-1 bg-zinc-900/80 p-2 rounded-xl border border-zinc-800/50 backdrop-blur-sm";
    }
  });

  const buttonClass = createMemo(() => {
    const isSalamander = instrumentName() === 'salamander';

    if (isSalamander) {
      return "px-3 py-1.5 rounded-lg text-[10px] font-bold transition-all duration-200 min-w-[45px] border border-transparent hover:border-zinc-600/50";
    } else {
      return "px-4 py-2 rounded-lg text-[11px] font-bold transition-all duration-200 min-w-[50px] border border-transparent hover:border-zinc-600/50";
    }
  });

  const getLayerColor = (_layer: string, isSelected: boolean) => {
    const isSalamander = instrumentName() === 'salamander';

    if (isSelected) {
      return isSalamander
        ? "bg-gradient-to-br from-purple-500 to-pink-500 text-white shadow-lg shadow-purple-500/30 border-purple-400/50"
        : "bg-gradient-to-br from-emerald-500 to-teal-500 text-white shadow-lg shadow-emerald-500/30 border-emerald-400/50";
    }

    return isSalamander
      ? "text-zinc-400 hover:text-purple-300 hover:bg-purple-500/10"
      : "text-zinc-400 hover:text-emerald-300 hover:bg-emerald-500/10";
  };

  const getLayerIcon = (layer: string) => {
    const isSalamander = instrumentName() === 'salamander';

    if (isSalamander) {
      const num = parseInt(layer.substring(1));
      if (num <= 4) return "🎵";
      if (num <= 8) return "🎶";
      if (num <= 12) return "🎼";
      return "🎧";
    } else {
      switch (layer) {
        case "PP": return "🎹";
        case "MP": return "🎵";
        case "MF": return "🎶";
        case "FF": return "🎼";
        default: return "🎹";
      }
    }
  };

  return (
    <div class={containerClass()}>
      {/* Layer indicator */}
      <div class="flex items-center gap-2 px-2 mr-2">
        <div class="text-xs text-zinc-500 font-medium uppercase tracking-wider">Velocity</div>
        <div class="w-1 h-4 bg-zinc-700 rounded-full"></div>
      </div>

      <For each={layers()}>{(layer) => (
        <button
          onClick={() => onLayerChange(layer)}
          onMouseEnter={() => onLayerHover?.(layer)}
          onMouseLeave={() => onLayerHover?.(null)}
          class={`${buttonClass()} ${getLayerColor(layer, selectedLayer() === layer)}`}
          title={`${layer} - ${instrumentName() === 'salamander' ? 'Velocity Layer' : 'Dynamic Layer'}`}
        >
          <div class="flex items-center gap-1">
            <span class="text-xs">{getLayerIcon(layer)}</span>
            <span>{layer}</span>
          </div>
        </button>
      )}</For>

      {/* Animated border effect for selected layer */}
      <div class="relative">
        <div
          class="absolute inset-0 bg-gradient-to-r from-emerald-500 to-purple-500 rounded-xl opacity-20 blur-sm transition-all duration-300"
          style={{
            width: selectedLayer() ? "60px" : "0px",
            height: "2px",
            "margin-top": "8px",
            "margin-left": selectedLayer() ? `${layers().indexOf(selectedLayer()) * 60}px` : "0px",
            transition: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)"
          }}
        ></div>
      </div>
    </div>
  );
}
