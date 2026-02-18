import Header from '@/components/header';
import Piano from '@/components/piano'
import LoadingIndicator from '@/components/loading'
import { usePiano } from "@/hooks/usePiano";

export default function Home() {
  const { 
    activeNotes, 
    selectedLayer,
    setSelectedLayer,
    hoveredLayer,
    setHoveredLayer,
    isLayerLoading,
    loadProgress,
    availableLayers,
    noteOn, 
    noteOff 
  } = usePiano();

  return (
    <div class="h-screen w-full bg-zinc-950 text-white flex flex-col justify-between overflow-hidden select-none font-sans">
      <Header
        selectedLayer={selectedLayer}
        onLayerChange={setSelectedLayer}
        onLayerHover={setHoveredLayer}
        layers={availableLayers}
      />

      <div class="flex-1 flex items-center justify-center opacity-20 pointer-events-none">
        <div class="text-6xl font-black italic text-zinc-800 tracking-tighter uppercase">Raku</div>
        
        {/* Enhanced Loading Indicator */}
        <LoadingIndicator 
          isLoading={isLayerLoading()} 
          instrument={"instrument"} 
          layer={selectedLayer()}
          progress={loadProgress()}
        />
        
        {/* Layer indicator on hover */}
        {hoveredLayer() && !isLayerLoading() && (
          <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-black/80 text-white px-4 py-2 rounded-lg shadow-lg">
            <div class="text-sm font-bold">Layer: {hoveredLayer()}</div>
            <div class="text-xs text-zinc-300">Velocity: {availableLayers().find((l: string) => l === hoveredLayer()) ? "High" : "Medium"}</div>
          </div>
        )}
      </div>

      <Piano
        activeNotes={activeNotes}
        onNoteOn={noteOn}
        onNoteOff={noteOff}
      />
    </div>
  );
}
