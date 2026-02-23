import { For } from "solid-js";
import type { InstrumentInfo } from "@/hooks/usePiano";

interface InstrumentInfoProps {
  instrument: InstrumentInfo;
}

export default function InstrumentInfo(props: InstrumentInfoProps) {
  return (
    <div class="bg-zinc-800 rounded-lg p-4 border border-zinc-700">
      <div class="grid grid-cols-2 gap-4 text-sm">
        {/* Basic Info */}
        <div class="space-y-2">
          <h3 class="font-semibold text-zinc-200 mb-2">Instrument</h3>
          <div class="space-y-1">
            <div class="flex justify-between">
              <span class="text-zinc-400">Name:</span>
              <span class="text-zinc-200 font-medium">{props.instrument.name}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-zinc-400">Format:</span>
              <span class="text-zinc-200 font-medium">{props.instrument.format}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-zinc-400">Keys:</span>
              <span class="text-zinc-200 font-medium">{props.instrument.layers.length} layers</span>
            </div>
          </div>
        </div>

        {/* Settings */}
        <div class="space-y-2">
          <h3 class="font-semibold text-zinc-200 mb-2">Settings</h3>
          <div class="space-y-1">
            <For each={props.instrument.settings || []}>{([key, value]) =>
              <div class="flex justify-between">
                <span class="text-zinc-400 capitalize">{key.replace(/_/g, ' ')}:</span>
                <span class="text-zinc-200 font-medium">{value}</span>
              </div>
            }</For>
          </div>
        </div>

        {/* Contribution */}
        <div class="space-y-2 col-span-2">
          <h3 class="font-semibold text-zinc-200 mb-2">Contribution</h3>
          <div class="space-y-1">
            <div class="flex justify-between">
              <span class="text-zinc-400">Authors:</span>
              <span class="text-zinc-200 font-medium">
                {props.instrument.contribution?.authors.join(", ") || "Unknown"}
              </span>
            </div>
            <div class="flex justify-between">
              <span class="text-zinc-400">Published:</span>
              <span class="text-zinc-200 font-medium">
                {props.instrument.contribution?.published_date || "Unknown"}
              </span>
            </div>
            <div class="flex justify-between">
              <span class="text-zinc-400">License:</span>
              <span class="text-zinc-200 font-medium">
                {props.instrument.contribution?.licenses.join(", ") || "Unknown"}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
