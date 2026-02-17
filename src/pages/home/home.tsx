import Header from "@/components/header";
import Piano from '@/components/piano'
import { usePiano } from "@/hooks/usePiano";

export default function Home() {
  const { activeNotes, selectedLayer, setSelectedLayer, layers, noteOn, noteOff } = usePiano();

  return (
    <div class="h-screen w-full bg-zinc-950 text-white flex flex-col justify-between overflow-hidden select-none font-sans">
      <Header
        selectedLayer={() => selectedLayer()}
        onLayerChange={setSelectedLayer}
        layers={layers}
      />

      <div class="flex-1 flex items-center justify-center opacity-20 pointer-events-none">
        <div class="text-6xl font-black italic text-zinc-800 tracking-tighter uppercase">Raku</div>
      </div>

      <Piano
        activeNotes={activeNotes}
        onNoteOn={noteOn}
        onNoteOff={noteOff}
      />
    </div>
  );
}
