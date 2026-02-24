import { Show } from "solid-js";
import type { LayerRange } from "../../hooks/usePiano";

interface LayerIndicatorProps {
  layerRanges: () => LayerRange[];
  leftLayerIdx: () => number;
  rightLayerIdx: () => number;
  leftOctave: () => number;
  rightOctave: () => number;
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

  // Convert octave offset to a human-readable octave label
  // Left base starts at A0 (octave 0 = A0–D2), right base starts at F4 (octave 0 = F4–B5)
  const leftOctaveLabel = () => {
    const o = props.leftOctave();
    return o >= 0 ? `+${o}` : `${o}`;
  };
  const rightOctaveLabel = () => {
    const o = props.rightOctave();
    return o >= 0 ? `+${o}` : `${o}`;
  };

  return (
    <Show when={props.layerRanges().length > 0}>
      <div class="flex items-center gap-2">
        {/* L label */}
        <span class="text-[1.2rem] text-blue-500 font-bold uppercase tracking-widest">
          L
        </span>

        {/* Left hand: layer box */}
        <Show
          when={leftLayerInfo()}
          fallback={
            <div class="flex items-center gap-1 px-2 py-1 rounded border border-zinc-800 bg-zinc-900">
              <span class="text-[0.8rem] font-black text-zinc-600">-</span>
              <span class="text-[0.8rem] text-zinc-600 tabular-nums opacity-50">
                --
              </span>
            </div>
          }
        >
          <div class="flex items-center gap-1 px-2 py-1 rounded border border-blue-500 bg-blue-900/60">
            <span class="text-[0.8rem] font-black text-blue-300">
              {leftLayerInfo()!.name}
            </span>
            <span class="text-[0.8rem] text-blue-400 tabular-nums opacity-80">
              {leftLayerInfo()!.lovel}-{leftLayerInfo()!.hivel}
            </span>
          </div>
        </Show>

        {/* Left octave badge */}
        <div class="flex items-center justify-center w-8 h-6 rounded border border-blue-700 bg-blue-950 select-none">
          <span class="text-[0.75rem] font-black text-blue-300 tabular-nums leading-none">
            {leftOctaveLabel()}
          </span>
        </div>

        {/* Divider */}
        <div class="h-4 w-px bg-zinc-700 mx-1" />

        {/* Right octave badge */}
        <div class="flex items-center justify-center w-8 h-6 rounded border border-green-700 bg-green-950 select-none">
          <span class="text-[0.75rem] font-black text-green-300 tabular-nums leading-none">
            {rightOctaveLabel()}
          </span>
        </div>

        {/* Right hand: layer box */}
        <Show
          when={rightLayerInfo()}
          fallback={
            <div class="flex items-center gap-1 px-2 py-1 rounded border border-zinc-800 bg-zinc-900">
              <span class="text-[0.8rem] font-black text-zinc-600">-</span>
              <span class="text-[0.8rem] text-zinc-600 tabular-nums opacity-50">
                --
              </span>
            </div>
          }
        >
          <div class="flex items-center gap-1 px-2 py-1 rounded border border-green-500 bg-green-900/60">
            <span class="text-[0.8rem] font-black text-green-300">
              {rightLayerInfo()!.name}
            </span>
            <span class="text-[0.8rem] text-green-400 tabular-nums opacity-80">
              {rightLayerInfo()!.lovel}-{rightLayerInfo()!.hivel}
            </span>
          </div>
        </Show>

        {/* R label */}
        <span class="text-[1.2rem] text-green-500 font-bold uppercase tracking-widest">
          R
        </span>
      </div>
    </Show>
  );
}
