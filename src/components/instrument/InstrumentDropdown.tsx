import { For, Show, createSignal } from "solid-js";
import type { InstrumentInfo } from "../../hooks/usePiano";

interface InstrumentDropdownProps {
  currentInstrument: () => InstrumentInfo | null;
  availableInstruments: () => InstrumentInfo[];
  isLoading: () => boolean;
  activeFolder: () => string | null;
  onSelectInstrument: (folder: string) => void;
}

export default function InstrumentDropdown(props: InstrumentDropdownProps) {
  const [showList, setShowList] = createSignal(false);

  const displayName = () => {
    const cur = props.currentInstrument();
    if (cur) return cur.name;
    return null;
  };

  return (
    <div class="relative">
      <Show when={displayName()} fallback={
        <button
          onClick={() => setShowList(true)}
          class="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 text-zinc-200 rounded-lg transition-colors"
        >
          Select Instrument
        </button>
      }>
        <div class="relative">
          <button
            onClick={() => setShowList(true)}
            class={`px-4 py-2 bg-zinc-800 hover:bg-zinc-700 text-zinc-200 rounded-lg transition-colors
              ${props.isLoading() ? "animate-pulse" : ""}`}
          >
            <span class="font-bold">{displayName()}</span>
          </button>

          <Show when={showList()}>
            <div class="absolute top-full left-0 mt-1 w-64 bg-zinc-800 rounded-lg shadow-xl border border-zinc-700 z-50 max-h-80 overflow-y-auto">
              <div class="p-1">
                <For each={props.availableInstruments()}>{inst => {
                  const isActive = () => props.activeFolder() === inst.folder;
                  const isLoading = () => isActive() && props.isLoading();
                  const isLoaded = () => isActive() && !props.isLoading();

                  return (
                    <button
                      onClick={() => {
                        props.onSelectInstrument(inst.folder);
                        setShowList(false);
                      }}
                      class={`w-full text-left px-3 py-2.5 border-b border-zinc-800/60
                              hover:bg-zinc-800 transition-colors group
                              ${isActive() ? "bg-zinc-800/50" : props.isLoading() && !isActive() ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`}
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
                            <span class="text-xs text-zinc-500">({inst.format})</span>
                          </div>
                        </div>
                      </div>
                    </button>
                  );
                }}</For>
              </div>
            </div>
          </Show>
        </div>
      </Show>
    </div>
  );
}
