import type { InstrumentInfo, LayerRange } from "../../hooks/usePiano";
import type { Modifier, SectionNum } from "../../utils/keyMapping";
import type {
  SongInfo,
  SessionStatus,
  SessionMode,
} from "../../hooks/useBuffer";
import InstrumentSelect from "./InstrumentSelect";
import LayerIndicator from "./LayerIndicator";

import SongSelect from "./SongSelect";
import { Show } from "solid-js";

interface HeaderContainerProps {
  // Instrument
  currentInstrument: () => InstrumentInfo | null;
  availableInstruments: () => InstrumentInfo[];
  availableLayers: () => string[];
  isLoading: () => boolean;
  loadProgress: () => number | null;
  activeFolder: () => string | null;
  onSelectInstrument: (folder: string) => void;
  leftSection: () => SectionNum | null;
  rightSection: () => SectionNum | null;
  leftModifier: () => Modifier;
  rightModifier: () => Modifier;
  leftLayerIdx: () => number;
  rightLayerIdx: () => number;
  availableLayerRanges: () => LayerRange[];
  velocityForLayer: (layer: string) => number;

  // Song
  availableSongs: () => SongInfo[];
  activeSong: () => SongInfo | null;
  isSongLoading: () => boolean;
  sessionStatus: () => SessionStatus;
  sessionMode: () => SessionMode;
  onSelectSong: (song: SongInfo) => void;
  onSelectMode: (mode: "perform" | "instruct") => void;
  onPauseResume: () => void;
  onStop: () => void;
}

export default function HeaderContainer(props: HeaderContainerProps) {
  const isPlaying = () => props.sessionStatus() === "playing";
  const isPaused = () => props.sessionStatus() === "paused";
  const showPauseResume = () => isPlaying() || isPaused();

  return (
    <header class="w-full bg-zinc-900 border-b border-zinc-800 px-4 py-3 relative">
      {/* ── True center: layer indicator absolutely centered in the window ── */}
      <div class="absolute inset-0 flex items-center justify-center pointer-events-none">
        <div class="pointer-events-auto">
          <LayerIndicator
            layerRanges={props.availableLayerRanges}
            leftLayerIdx={props.leftLayerIdx}
            rightLayerIdx={props.rightLayerIdx}
            leftActive={() => props.leftSection() !== null}
            rightActive={() => props.rightSection() !== null}
          />
        </div>
      </div>

      <div class="flex items-center justify-between gap-4">
        {/* ── Left: instrument selector ── */}
        <div class="flex items-center gap-3 flex-shrink-0">
          <InstrumentSelect
            currentInstrument={props.currentInstrument}
            availableInstruments={props.availableInstruments}
            isLoading={props.isLoading}
            loadProgress={props.loadProgress}
            activeFolder={props.activeFolder}
            onSelectInstrument={props.onSelectInstrument}
          />
        </div>

        {/* ── Spacer so justify-between still pushes right section to the edge ── */}
        <div class="flex-1" />

        {/* ── Right: song selector + pause/resume ── */}
        <div class="flex items-center gap-2 flex-shrink-0">
          {/* Pause / Resume button — only visible when playing or paused */}
          <Show when={showPauseResume()}>
            <button
              onClick={props.onPauseResume}
              class={`flex items-center justify-center w-8 h-8 rounded-lg border transition-colors ${
                isPaused()
                  ? "bg-blue-600 border-blue-500 text-white hover:bg-blue-500"
                  : "bg-zinc-800 border-zinc-700 text-zinc-300 hover:bg-zinc-700"
              }`}
              title={isPaused() ? "Resume" : "Pause"}
            >
              <span class="text-sm leading-none">{isPaused() ? "▶" : "⏸"}</span>
            </button>
          </Show>

          <SongSelect
            availableSongs={props.availableSongs}
            activeSong={props.activeSong}
            isLoading={props.isSongLoading}
            sessionStatus={props.sessionStatus}
            sessionMode={props.sessionMode}
            onSelectSong={props.onSelectSong}
            onSelectMode={props.onSelectMode}
            onStop={props.onStop}
          />
        </div>
      </div>
    </header>
  );
}
