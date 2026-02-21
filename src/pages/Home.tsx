import HeaderContainer from '@/components/header/HeaderContainer';
import PianoContainer from '@/components/piano/PianoContainer';
import { usePiano } from "@/hooks/usePiano";

export default function Home() {
  const piano = usePiano();

  return (
    <div class="h-screen w-full bg-zinc-950 text-white flex flex-col overflow-hidden select-none font-sans">
      <HeaderContainer
        currentInstrument={piano.currentInstrument}
        availableInstruments={piano.availableInstruments}
        isLoading={piano.isLoading}
        loadProgress={piano.loadProgress}
        activeFolder={piano.activeFolder}
        onSelectInstrument={piano.selectInstrument}
        leftSection={piano.leftSection}
        rightSection={piano.rightSection}
        leftModifier={piano.leftModifier}
        rightModifier={piano.rightModifier}
        leftLayerIdx={piano.leftLayerIdx}
        rightLayerIdx={piano.rightLayerIdx}
        availableLayers={piano.availableLayers}
        availableLayerRanges={piano.availableLayerRanges}
        velocityForLayer={piano.velocityForLayer}
      />

      <div class="flex-1 flex items-center justify-center pointer-events-none">
        <div class="text-6xl font-black italic text-zinc-800 tracking-tighter uppercase opacity-20">
          Rakund
        </div>
      </div>

      <PianoContainer
        activeNotes={piano.activeNotes}
        onNoteOn={piano.noteOn}
        onNoteOff={piano.noteOff}
        leftSection={piano.leftSection}
        rightSection={piano.rightSection}
      />
    </div>
  );
}
