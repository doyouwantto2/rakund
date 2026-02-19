import { For, Show, createSignal } from "solid-js";
import type { InstrumentInfo } from "../hooks/usePiano";
import type { Modifier } from "../utils/keyMapping";

interface HeaderProps {
  currentInstrument: () => InstrumentInfo | null;
  availableInstruments: () => InstrumentInfo[];
  availableLayers: () => string[];
  leftLayerIdx: () => number;
  rightLayerIdx: () => number;
  volume: () => number;
  onVolumeChange: (v: number) => void;
  isLoading: () => boolean;
  loadProgress: () => number | null;
  onSelectInstrument: (folder: string) => void;
  leftModifier: () => Modifier;
  rightModifier: () => Modifier;
}

export default function Header(props: HeaderProps) {
  const [showList, setShowList] = createSignal(false);

  const progressPct = () => {
    const p = props.loadProgress();
    return p !== null ? Math.max(0, Math.min(100, p)) : 0;
  };

  return (
    <div class="relative shrink-0" onMouseLeave={() => setShowList(false)}>
      <header class="flex items-center gap-3 px-4 bg-zinc-900 border-b border-zinc-800 h-10 overflow-hidden">
        <span class="text-sm font-black italic tracking-tight text-zinc-300 shrink-0">Raku</span>
        <div class="w-px h-4 bg-zinc-700 shrink-0" />

        <button
          onClick={() => setShowList(v => !v)}
          class={`flex items-center gap-1.5 text-xs px-2.5 py-1 rounded border transition-colors shrink-0 ${props.currentInstrument()
            ? "border-zinc-700 bg-zinc-800 text-zinc-200 hover:border-zinc-500 cursor-pointer"
            : "border-dashed border-zinc-600 text-zinc-500 hover:border-zinc-400 hover:text-zinc-300 cursor-pointer"
            }`}
        >
          <Show when={props.currentInstrument()}>
            <span class={`w-1.5 h-1.5 rounded-full shrink-0 ${props.isLoading() ? "bg-amber-400 animate-pulse" : "bg-emerald-400"
              }`} />
          </Show>

          <Show
            when={props.currentInstrument()}
            fallback={
              <span class="text-zinc-500">
                {props.isLoading() ? "Loading…" : "Select instrument"}
              </span>
            }
          >
            <span class="font-bold truncate max-w-[160px]">{props.currentInstrument()?.name}</span>
            <Show when={props.isLoading()}>
              <span class="text-[9px] text-amber-400 font-bold shrink-0">
                {progressPct().toFixed(0)}%
              </span>
            </Show>
            <span class="text-[9px] px-1 py-0.5 rounded bg-zinc-700 text-zinc-400 uppercase font-bold shrink-0">
              {props.currentInstrument()?.format}
            </span>
          </Show>

          <span class="text-zinc-500 text-[9px] shrink-0">{showList() ? "▴" : "▾"}</span>
        </button>

        <div class="w-px h-4 bg-zinc-700 shrink-0" />

        <div class="flex items-center gap-2 ml-auto">
          <span class="text-[10px] text-zinc-500 font-bold shrink-0">VOL</span>
          <input
            type="range" min="0" max="1" step="0.01"
            value={props.volume()}
            onInput={e => props.onVolumeChange(parseFloat(e.currentTarget.value))}
            class="w-20 h-1 accent-zinc-400 cursor-pointer"
          />
          <span class="text-[10px] text-zinc-500 w-6 text-right shrink-0">
            {Math.round(props.volume() * 100)}
          </span>
        </div>
      </header>

      <Show when={showList()}>
        <div class="absolute top-full left-0 z-50 min-w-[320px] max-w-[420px]
                    bg-zinc-900 border border-zinc-800 border-t-0
                    shadow-2xl shadow-black/60 max-h-[60vh] overflow-y-auto">

          <Show
            when={props.availableInstruments().length > 0}
            fallback={
              <div class="px-5 py-6 text-center">
                <p class="text-zinc-400 text-sm font-bold">No instruments found</p>
                <p class="text-zinc-600 text-xs mt-1 font-mono">~/.config/raku/instruments/</p>
              </div>
            }
          >
            <div class="px-3 py-1.5 border-b border-zinc-800 flex items-center justify-between">
              <span class="text-[10px] text-zinc-500 font-bold uppercase tracking-widest">
                {props.availableInstruments().length} available
              </span>
              <Show when={props.isLoading()}>
                <span class="text-[10px] text-amber-400 font-bold animate-pulse">
                  loading {progressPct().toFixed(0)}%
                </span>
              </Show>
            </div>

            <For each={props.availableInstruments()}>{inst => {
              const isLoaded = () => props.currentInstrument()?.folder === inst.folder;
              const isCurrentlyLoading = () => isLoaded() && props.isLoading();
              return (
                <button
                  onClick={() => {
                    // GUARD: Do nothing if already loaded/loading
                    if (isLoaded()) {
                      setShowList(false);
                      return;
                    }
                    // Trigger backend load for new instrument
                    props.onSelectInstrument(inst.folder);
                    setShowList(false);
                  }}
                  class={`w-full text-left px-3 py-2.5 border-b border-zinc-800/60
                          hover:bg-zinc-800 transition-colors group
                          ${isLoaded() ? "bg-zinc-800/50 cursor-default" : "cursor-pointer"}`}
                >
                  <div class="flex items-center gap-2.5">
                    <span class={`w-2 h-2 rounded-full shrink-0 transition-colors ${isCurrentlyLoading()
                      ? "bg-amber-400 animate-pulse"
                      : isLoaded()
                        ? "bg-emerald-400 shadow-[0_0_6px_2px_rgba(52,211,153,0.4)]"
                        : "bg-emerald-700 group-hover:bg-emerald-600"
                      }`} />

                    <div class="flex-1 min-w-0">
                      <div class="flex items-center gap-1.5">
                        <span class="text-sm font-bold text-zinc-200 truncate">{inst.name}</span>
                        <span class="text-[9px] px-1 py-0.5 rounded bg-zinc-700 text-zinc-400 uppercase font-bold shrink-0">
                          {inst.format}
                        </span>
                        <Show when={isCurrentlyLoading()}>
                          <span class="text-[9px] text-amber-400 font-bold shrink-0 animate-pulse">
                            {progressPct().toFixed(0)}%
                          </span>
                        </Show>
                        <Show when={isLoaded() && !props.isLoading()}>
                          <span class="text-[9px] text-emerald-400 font-bold shrink-0">LOADED</span>
                        </Show>
                      </div>

                      <div class="flex items-center gap-1 mt-0.5">
                        <For each={inst.layers}>{layer => (
                          <span class="text-[9px] font-bold px-1 py-0.5 rounded
                                       bg-zinc-800 border border-zinc-700 text-zinc-500">
                            {layer}
                          </span>
                        )}</For>
                      </div>

                      <Show when={isCurrentlyLoading()}>
                        <div class="mt-1.5 h-0.5 bg-zinc-700 rounded-full overflow-hidden">
                          <div
                            class="h-full bg-amber-400 transition-all duration-200"
                            style={`width: ${progressPct()}%`}
                          />
                        </div>
                      </Show>
                    </div>

                    <span class="text-zinc-600 group-hover:text-zinc-300 transition-colors shrink-0 text-xs">→</span>
                  </div>
                </button>
              );
            }}</For>
          </Show>
        </div>
      </Show>
    </div>
  );
}
