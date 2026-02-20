import { For, Show, createSignal } from "solid-js";
import type { InstrumentInfo } from "../../hooks/usePiano";

interface InstrumentSelectProps {
  currentInstrument: () => InstrumentInfo | null;
  availableInstruments: () => InstrumentInfo[];
  isLoading: () => boolean;
  loadProgress: () => number | null;
  activeFolder: () => string | null;
  onSelectInstrument: (folder: string) => void;
}

export default function InstrumentSelect(props: InstrumentSelectProps) {
  const [showList, setShowList] = createSignal(false);

  const progressPct = () => {
    const p = props.loadProgress();
    return p !== null ? Math.max(0, Math.min(100, p)) : 0;
  };

  const displayName = () => {
    const af = props.activeFolder();
    if (af && props.isLoading()) {
      const match = props.availableInstruments().find(i => i.folder === af);
      return match?.name ?? af;
    }

    const cur = props.currentInstrument();
    if (cur) return cur.name;

    return null;
  };

  const displayFormat = () => {
    const af = props.activeFolder();
    if (af && props.isLoading()) {
      const match = props.availableInstruments().find(i => i.folder === af);
      return match?.format ?? "";
    }

    const cur = props.currentInstrument();
    if (cur) return cur.format;

    return "";
  };

  return (
    <div class="flex-shrink-0 relative">
      {/* Instrument button */}
      <button
        onClick={() => setShowList(v => !v)}
        class={`flex items-center gap-2 bg-zinc-800 rounded-lg px-15 py-1.5 border border-zinc-700 transition-colors shrink-0 cursor-pointer hover:bg-zinc-700 ${displayName()
          ? "text-zinc-200"
          : "text-zinc-400"
          }`}
      >
        <Show when={displayName()}>
          <span class={`w-1.5 h-1.5 rounded-full shrink-0 ${props.isLoading() ? "bg-amber-400 animate-pulse" : "bg-emerald-400"}`} />
          <span class="font-medium truncate max-w-[160px]">{displayName()}</span>
          <Show when={props.isLoading()}>
            <span class="text-xs text-amber-400 font-bold shrink-0">
              {progressPct().toFixed(0)}%
            </span>
          </Show>
          <span class="text-xs px-1.5 py-0.5 rounded bg-zinc-700 text-zinc-400 uppercase font-medium shrink-0">
            {displayFormat()}
          </span>
        </Show>
        <Show when={!displayName()}>
          <span class="text-zinc-400">Select instrument</span>
        </Show>
        <span class="text-zinc-400 text-xs shrink-0">{showList() ? "▴" : "▾"}</span>
      </button>

      {/* Instrument Dropdown */}
      <Show when={showList()}>
        <div class="absolute top-full left-0 mt-2 z-50 w-full bg-zinc-800 rounded-lg shadow-xl border border-zinc-700 max-h-[60vh] overflow-y-auto">
          <div class="p-1">
            <For each={props.availableInstruments()}>{inst => {
              const isActive = () => props.activeFolder() === inst.folder;
              const isLoading = () => isActive() && props.isLoading();
              const isLoaded = () => isActive() && !props.isLoading();
              const isOtherLoading = () => !isActive() && props.isLoading();

              return (
                <div class="mb-1 border-b border-zinc-700/60 last:border-b-0">
                  <button
                    onClick={() => {
                      if (isOtherLoading()) return;
                      props.onSelectInstrument(inst.folder);
                      setShowList(false);
                    }}
                    disabled={isOtherLoading()}
                    class={`w-full text-left px-4 py-4 pt-5 transition-colors group ${isActive() ? "bg-zinc-700/50" : isOtherLoading() ? "opacity-50 cursor-not-allowed" : "cursor-pointer hover:bg-zinc-800"}`}
                  >
                    <div class="flex items-center gap-3">
                      <span class={`w-2 h-2 rounded-full shrink-0 transition-colors ${isLoading()
                        ? "bg-amber-400 animate-pulse"
                        : isLoaded()
                          ? "bg-emerald-400 shadow-[0_0_6px_2px_rgba(52,211,153,0.4)]"
                          : "bg-emerald-700 group-hover:bg-emerald-600"
                        }`} />

                      <div class="flex-1 min-w-0">
                        <div class="flex items-center gap-2">
                          <span class="text-sm font-medium text-zinc-200 truncate">{inst.name}</span>
                          <span class="text-xs px-2 py-1 rounded bg-zinc-700 text-zinc-400 uppercase font-medium shrink-0">
                            {inst.format}
                          </span>
                          <Show when={isLoading()}>
                            <span class="text-xs text-amber-400 font-medium shrink-0 animate-pulse">
                              {progressPct().toFixed(0)}%
                            </span>
                          </Show>
                          <Show when={isLoaded()}>
                            <span class="text-xs text-emerald-400 font-medium shrink-0">LOADED</span>
                          </Show>
                        </div>
                      </div>

                      <span class="text-zinc-500 group-hover:text-zinc-300 transition-colors shrink-0">→</span>
                    </div>
                  </button>

                  {/* Layers with horizontal scroll and scrollbar beneath */}
                  <div class="px-4 pb-6 max-h-40 overflow-x-auto scrollbar-thin scrollbar-thumb-zinc-600 scrollbar-track-zinc-800">
                    <div class="flex gap-0.5 mt-2">
                      <For each={inst.layers}>{layer => (
                        <span class="text-xs font-medium px-2 py-1 rounded bg-zinc-700 border border-zinc-600 text-zinc-400 whitespace-nowrap">
                          {layer}
                        </span>
                      )}</For>
                    </div>
                  </div>

                  {/* Loading Progress */}
                  <Show when={isLoading()}>
                    <div class="px-4 pb-3">
                      <div class="h-1 bg-zinc-700 rounded-full overflow-hidden">
                        <div
                          class="h-full bg-amber-400 transition-all duration-200"
                          style={{ width: `${progressPct()}%` }}
                        />
                      </div>
                    </div>
                  </Show>
                </div>
              );
            }}</For>
          </div>
        </div>
      </Show>
    </div>
  );
}
