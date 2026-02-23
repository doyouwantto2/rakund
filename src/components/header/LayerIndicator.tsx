import { Show } from "solid-js";
import type { LayerRange } from "../../hooks/usePiano";

interface LayerIndicatorProps {
  layerRanges: () => LayerRange[];
  leftLayerIdx: () => number;
  rightLayerIdx: () => number;
  leftActive: () => boolean;
  rightActive: () => boolean;
}

export default function LayerIndicator(props: LayerIndicatorProps) {
  const getLayerInfo = (layerIdx: number) => {
    const layers = props.layerRanges();
    if (layerIdx >= 0 && layerIdx < layers.length) {
      return layers[layerIdx];
    }
    return null;
  };

  const leftLayerInfo = () => getLayerInfo(props.leftLayerIdx());
  const rightLayerInfo = () => getLayerInfo(props.rightLayerIdx());

  return (
    <Show when={props.layerRanges().length > 0}>
      <div class="flex items-center gap-2">
        {/* L label — far left */}
        <span class="text-[1.2rem] text-blue-500 font-bold uppercase tracking-widest">
          L
        </span>

        {/* Left hand box */}
        <Show when={props.leftActive() && leftLayerInfo()}>
          <div class="flex items-center gap-1 px-2 py-1 rounded border border-blue-500 bg-blue-900/60">
            <span class="text-[0.8rem] font-black text-blue-300">
              {leftLayerInfo()!.name}
            </span>
            <span class="text-[0.8rem] text-blue-400 tabular-nums opacity-80">
              {leftLayerInfo()!.lovel}-{leftLayerInfo()!.hivel}
            </span>
          </div>
        </Show>
        <Show when={!props.leftActive()}>
          <div class="flex items-center gap-1 px-2 py-1 rounded border border-zinc-800 bg-zinc-900">
            <span class="text-[0.8rem] font-black text-zinc-600">-</span>
            <span class="text-[0.8rem] text-zinc-600 tabular-nums opacity-50">
              --
            </span>
          </div>
        </Show>

        {/* Right hand box */}
        <Show when={props.rightActive() && rightLayerInfo()}>
          <div class="flex items-center gap-1 px-2 py-1 rounded border border-green-500 bg-green-900/60">
            <span class="text-[0.8rem] font-black text-green-300">
              {rightLayerInfo()!.name}
            </span>
            <span class="text-[0.8rem] text-green-400 tabular-nums opacity-80">
              {rightLayerInfo()!.lovel}-{rightLayerInfo()!.hivel}
            </span>
          </div>
        </Show>
        <Show when={!props.rightActive()}>
          <div class="flex items-center gap-1 px-2 py-1 rounded border border-zinc-800 bg-zinc-900">
            <span class="text-[0.8rem] font-black text-zinc-600">-</span>
            <span class="text-[0.8rem] text-zinc-600 tabular-nums opacity-50">
              --
            </span>
          </div>
        </Show>

        {/* R label — far right */}
        <span class="text-[1.2rem] text-green-500 font-bold uppercase tracking-widest">
          R
        </span>
      </div>
    </Show>
  );
}
