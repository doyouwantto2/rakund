import { For, Show } from "solid-js";
import type { LayerRange } from "../../hooks/usePiano";

interface LayerIndicatorProps {
  layerRanges: () => LayerRange[];
  leftLayerIdx: () => number;
  rightLayerIdx: () => number;
  leftActive: () => boolean;
  rightActive: () => boolean;
}

export default function LayerIndicator(props: LayerIndicatorProps) {
  return (
    <Show when={props.layerRanges().length > 0}>
      <div class="flex items-center gap-1">
        <For each={props.layerRanges()}>{(range, idx) => {
          const isLeft = () => props.leftActive() && props.leftLayerIdx() === idx();
          const isRight = () => props.rightActive() && props.rightLayerIdx() === idx();
          const isBoth = () => isLeft() && isRight();

          const style = () => {
            if (isBoth()) return "bg-purple-700/60 border-purple-500 text-white";
            if (isLeft()) return "bg-blue-700/60   border-blue-500   text-white";
            if (isRight()) return "bg-green-700/60  border-green-500  text-white";
            return "bg-zinc-900 border-zinc-800 text-zinc-600";
          };

          return (
            <div class={`flex flex-col items-center justify-center w-10 h-9 rounded border transition-colors duration-75 ${style()}`}>
              <span class="text-[9px] font-black leading-none">{range.name}</span>
              <span class="text-[7px] leading-none mt-0.5 tabular-nums opacity-70">
                {range.lovel}–{range.hivel}
              </span>
            </div>
          );
        }}</For>
      </div>
    </Show>
  );
}
