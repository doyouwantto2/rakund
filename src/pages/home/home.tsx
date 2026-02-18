import Header from '@/components/header';
import Piano from '@/components/piano';
import LoadingIndicator from '@/components/loading';
import { usePiano } from "@/hooks/usePiano";

export default function Home() {
  const {
    activeNotes,
    selectedLayer,
    setSelectedLayer,
    setHoveredLayer,
    isLayerLoading,
    loadProgress,
    availableLayers,
    noteOn,
    noteOff,
    leftSection,
    rightSection,
    getSemitoneOffset,
  } = usePiano();

  return (
    <div class="h-screen w-full bg-zinc-950 text-white flex flex-col overflow-hidden select-none font-sans">
      <Header
        selectedLayer={selectedLayer}
        onLayerChange={setSelectedLayer}
        onLayerHover={setHoveredLayer}
        layers={availableLayers}
      />

      <div class="flex-1 flex items-center justify-center pointer-events-none">
        <div class="text-6xl font-black italic text-zinc-800 tracking-tighter uppercase opacity-20">
          Raku
        </div>
      </div>

      <LoadingIndicator
        isLoading={isLayerLoading()}
        instrument="instrument"
        layer={selectedLayer()}
        progress={loadProgress()}
      />

      <Piano
        activeNotes={activeNotes}
        onNoteOn={noteOn}
        onNoteOff={noteOff}
        leftSection={leftSection}
        rightSection={rightSection}
      />
    </div>
  );
}
