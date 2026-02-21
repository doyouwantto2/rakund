import type { InstrumentInfo, LayerRange } from "../../hooks/usePiano";
import type { Modifier, SectionNum } from "../../utils/keyMapping";
import InstrumentSelect from "./InstrumentSelect";
import LayerIndicator from "./LayerIndicator";

interface HeaderContainerProps {
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
}

export default function HeaderContainer(props: HeaderContainerProps) {
  return (
    <header class="w-full bg-zinc-900 border-b border-zinc-800 px-4 py-3">
      <div class="flex items-center justify-between gap-4">

        {/* ── Left: instrument selector ── */}
        <InstrumentSelect
          currentInstrument={props.currentInstrument}
          availableInstruments={props.availableInstruments}
          isLoading={props.isLoading}
          loadProgress={props.loadProgress}
          activeFolder={props.activeFolder}
          onSelectInstrument={props.onSelectInstrument}
        />

        {/* ── Right: velocity layer indicator ── */}
        <div class="flex items-center gap-3">
          {/* Layer indicator */}
          <LayerIndicator
            layerRanges={props.availableLayerRanges}
            leftLayerIdx={props.leftLayerIdx}
            rightLayerIdx={props.rightLayerIdx}
            leftActive={() => props.leftSection() !== null}
            rightActive={() => props.rightSection() !== null}
          />
        </div>
      </div>
    </header>
  );
}
