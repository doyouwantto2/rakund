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
  activeFolder: () => string | null;
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

  // What to show as the instrument name in the button:
  // - If loaded, show currentInstrument name
  // - If loading but not yet loaded, show name from available list
  const displayName = () => {
    const cur = props.currentInstrument();
    if (cur) return cur.name;
    const af = props.activeFolder();
    if (af && props.isLoading()) {
      const match = props.availableInstruments().find(i => i.folder === af);
      return match?.name ?? af;
    }
    return null;
  };

  const displayFormat = () => {
    const cur = props.currentInstrument();
    if (cur) return cur.format;
    const af = props.activeFolder();
    if (af) {
      const match = props.availableInstruments().find(i => i.folder === af);
      return match?.format ?? "";
    }
    return "";
  };

  return (
    <div class="relative shrink-0" onMouseLeave={() => setShowList(false)}>
      <header class="flex items-center gap-3 px-4 bg-zinc-900 border-b border-zinc-800 h-10 overflow-hidden relative">

        <span class="text-sm font-black italic tracking-tight text-zinc-300 shrink-0">Rakund</span>
        <div class="w-px h-4 bg-zinc-700 shrink-0" />

        {/* Instrument button */}
        <button
          onClick={() => setShowList(v => !v)}
          class={`flex items-center gap-1.5 text-xs px-2.5 py-1 rounded border transition-colors shrink-0 cursor-pointer ${displayName()
            ? "border-zinc-700 bg-zinc-800 text-zinc-200 hover:border-zinc-500"
            : "border-dashed border-zinc-600 text-zinc-500 hover:border-zinc-400 hover:text-zinc-300"
            }`}
        >
          <Show when={displayName()}>
            <span class={`w-1.5 h-1.5 rounded-full shrink-0 ${props.isLoading() ? "bg-amber-400 animate-pulse" : "bg-emerald-400"
              }`} />
            <span class="font-bold truncate max-w-[160px]">{displayName()}</span>
            <Show when={props.isLoading()}>
              <span class="text-[9px] text-amber-400 font-bold shrink-0">
                {progressPct().toFixed(0)}%
              </span>
            </Show>
            <span class="text-[9px] px-1 py-0.5 rounded bg-zinc-700 text-zinc-400 uppercase font-bold shrink-0">
              {displayFormat()}
            </span>
          </Show>
          <Show when={!displayName()}>
            <span class="text-zinc-500">Select instrument</span>
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

      {/* Dropdown */}
      <Show when={showList()}>
        <div class="absolute top-full left-0 z-50 min-w-[320px] max-w-[420px]
                    bg-zinc-900 border border-zinc-800 border-t-0
                    shadow-2xl shadow-black/60 max-h-[60vh] overflow-y-auto">

          <Show
            when={props.availableInstruments().length > 0}
            fallback={
              <div class="px-5 py-6 text-center">
                <p class="text-zinc-400 text-sm font-bold">No instruments found</p>
                <p class="text-zinc-600 text-xs mt-1 font-mono">~/.config/rakund/instruments/</p>
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
              // Use activeFolder — not currentInstrument — as the source of truth
              const isActive = () => props.activeFolder() === inst.folder;
              const isLoading = () => isActive() && props.isLoading();
              const isLoaded = () => isActive() && !props.isLoading();

              return (
                <button
                  onClick={() => {
                    props.onSelectInstrument(inst.folder); // guard is in usePiano
                    setShowList(false);
                  }}
                  class={`w-full text-left px-3 py-2.5 border-b border-zinc-800/60
                          hover:bg-zinc-800 transition-colors group
                          ${isActive() ? "bg-zinc-800/50" : "cursor-pointer"}`}
                >
                  <div class="flex items-center gap-2.5">
                    <span class={`w-2 h-2 rounded-full shrink-0 transition-colors ${isLoading()
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
                        <Show when={isLoading()}>
                          <span class="text-[9px] text-amber-400 font-bold shrink-0 animate-pulse">
                            {progressPct().toFixed(0)}%
                          </span>
                        </Show>
                        <Show when={isLoaded()}>
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

                      <Show when={isLoading()}>
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
