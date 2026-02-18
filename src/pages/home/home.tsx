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
    leftAlt, rightAlt, leftShift, rightShift,
    getSemitoneOffset,
  } = usePiano();

  const modifierLabel = () => {
    const o = getSemitoneOffset();
    if (o === 2) return { text: "+2 semitones (𝄪)", color: "text-emerald-300" };
    if (o === 1) return { text: "+1 semitone (♯)", color: "text-emerald-400" };
    if (o === -1) return { text: "−1 semitone (♭)", color: "text-blue-400" };
    if (o === -2) return { text: "−2 semitones (𝄫)", color: "text-blue-300" };
    return null;
  };

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

      {/* Modifier label */}
      {modifierLabel() && (
        <div class="absolute top-24 left-1/2 -translate-x-1/2 pointer-events-none z-30">
          <div class={`text-sm font-bold px-3 py-1 rounded-lg bg-zinc-900/90 border border-zinc-700 ${modifierLabel()!.color}`}>
            {modifierLabel()!.text}
          </div>
        </div>
      )}

      {/* Modifier key pills */}
      <div class="flex justify-center gap-2 mb-1 text-[10px] font-bold pointer-events-none">
        <span class={`px-2 py-0.5 rounded border ${leftAlt() ? "bg-blue-900 border-blue-500 text-blue-300" : "border-zinc-800 text-zinc-700"}`}>L-Alt</span>
        <span class={`px-2 py-0.5 rounded border ${rightAlt() ? "bg-blue-900 border-blue-500 text-blue-300" : "border-zinc-800 text-zinc-700"}`}>R-Alt</span>
        <span class={`px-2 py-0.5 rounded border ${leftShift() ? "bg-emerald-900 border-emerald-500 text-emerald-300" : "border-zinc-800 text-zinc-700"}`}>L-Shift</span>
        <span class={`px-2 py-0.5 rounded border ${rightShift() ? "bg-emerald-900 border-emerald-500 text-emerald-300" : "border-zinc-800 text-zinc-700"}`}>R-Shift</span>
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
