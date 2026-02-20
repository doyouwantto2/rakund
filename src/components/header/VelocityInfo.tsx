import { Show } from "solid-js";
import type { Modifier } from "../../utils/keyMapping";

interface VelocityInfoProps {
  leftModifier: () => Modifier;
  rightModifier: () => Modifier;
  leftLayerIdx: () => number;
  rightLayerIdx: () => number;
  availableLayers: () => string[];
  velocityForLayer: (layer: string) => number;
}

function modBadge(mod: Modifier) {
  if (!mod) return null;
  return (
    <span class={`text-[8px] font-black ml-2 ${mod === 'sharp' ? 'text-emerald-400' : 'text-blue-400'}`}>
      {mod === 'sharp' ? '♯' : '♭'}
    </span>
  );
}

export default function VelocityInfo(props: VelocityInfoProps) {
  const leftVelLabel = () => {
    const layers = props.availableLayers();
    if (layers.length === 0) return "—";
    const layer = layers[Math.min(props.leftLayerIdx(), layers.length - 1)];
    const vel = props.velocityForLayer(layer);
    return `${layer} ${vel}`;
  };

  const rightVelLabel = () => {
    const layers = props.availableLayers();
    if (layers.length === 0) return "—";
    const layer = layers[Math.min(props.rightLayerIdx(), layers.length - 1)];
    const vel = props.velocityForLayer(layer);
    return `${layer} ${vel}`;
  };

  return (
    <div class="flex items-center gap-2 text-[12px]">
      <Show when={props.availableLayers().length > 0}>
        <div class="flex items-center gap-0.5">
          <span class="text-blue-500 font-black">L</span>
          <span class="text-zinc-300 font-bold">{leftVelLabel()}</span>
          {modBadge(props.leftModifier())}
        </div>
        <div class="flex items-center gap-0.5">
          <span class="text-green-500 font-black">R</span>
          <span class="text-zinc-300 font-bold">{rightVelLabel()}</span>
          {modBadge(props.rightModifier())}
        </div>
      </Show>
    </div>
  );
}
