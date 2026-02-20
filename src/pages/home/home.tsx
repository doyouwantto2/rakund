import Header from '@/components/header';
import PianoSection from '@/components/pianosection';
import { usePiano } from "@/hooks/usePiano";

export default function Home() {
  const piano = usePiano();

  return (
    <div class="h-screen w-full bg-zinc-950 text-white flex flex-col overflow-hidden select-none font-sans">

      <Header
        currentInstrument={piano.currentInstrument}
        availableInstruments={piano.availableInstruments}
        availableLayers={piano.availableLayers}
        leftLayerIdx={piano.leftLayerIdx}
        rightLayerIdx={piano.rightLayerIdx}
        volume={piano.volume}
        onVolumeChange={piano.setVolume}
        isLoading={piano.isLoading}
        loadProgress={piano.loadProgress}
        activeFolder={piano.activeFolder}
        onSelectInstrument={piano.selectInstrument}
        leftModifier={piano.leftModifier}
        rightModifier={piano.rightModifier}
      />

      <div class="flex-1 flex items-center justify-center pointer-events-none">
        <div class="text-6xl font-black italic text-zinc-800 tracking-tighter uppercase opacity-20">
          Rakund
        </div>
      </div>

      <PianoSection
        activeNotes={piano.activeNotes}
        onNoteOn={piano.noteOn}
        onNoteOff={piano.noteOff}
        leftSection={piano.leftSection}
        rightSection={piano.rightSection}
        leftModifier={piano.leftModifier}
        rightModifier={piano.rightModifier}
        leftLayerIdx={piano.leftLayerIdx}
        rightLayerIdx={piano.rightLayerIdx}
        availableLayers={piano.availableLayers}
        velocityForLayer={piano.velocityForLayer}
      />

    </div>
  );
}
