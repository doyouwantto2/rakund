import VelocitySelector from "./velocity";
import VolumeControl from "./volume";

interface HeaderProps {
  selectedInstrument: () => string;
  selectedLayer: () => string;
  onInstrumentChange: (instrument: string) => void;
  onLayerChange: (layer: string) => void;
  onLayerHover?: (layer: string | null) => void;
  layers: () => string[];
}

export default function Header(props: HeaderProps) {
  const { selectedInstrument, selectedLayer, onInstrumentChange, onLayerChange, onLayerHover, layers } = props;
  
  return (
    <div class="shrink-0 p-6 flex items-center justify-between border-b border-zinc-900 bg-gradient-to-r from-zinc-950 via-zinc-900 to-zinc-950 backdrop-blur-sm">
      <div class="flex items-center gap-8">
        {/* Logo */}
        <div class="flex items-center gap-4">
          <div class="relative">
            <div class="w-12 h-12 bg-gradient-to-br from-emerald-500 to-emerald-600 rounded-xl flex items-center justify-center shadow-lg shadow-emerald-500/20">
              <div class="text-white font-black text-xl">R</div>
            </div>
            <div class="absolute -top-1 -right-1 w-3 h-3 bg-emerald-400 rounded-full animate-pulse"></div>
          </div>
          <div>
            <h1 class="text-2xl font-black italic text-emerald-500 tracking-tighter uppercase leading-none">Raku Grand</h1>
            <p class="text-xs text-zinc-500 font-medium">Multi-Instrument Piano</p>
          </div>
        </div>
        
        {/* Instrument Selector */}
        <div class="flex items-center gap-3">
          <div class="relative">
            <div class="absolute inset-0 bg-gradient-to-r from-purple-500 to-pink-500 rounded-xl blur opacity-20"></div>
            <select
              value={selectedInstrument()}
              onChange={(e) => onInstrumentChange(e.target.value)}
              class='relative bg-zinc-800/80 text-zinc-100 px-4 py-2.5 rounded-xl border border-zinc-700/50 focus:outline-none focus:ring-2 focus:ring-emerald-500/50 focus:border-emerald-500/50 backdrop-blur-sm appearance-none cursor-pointer transition-all duration-200 hover:bg-zinc-800/90 min-w-[140px]'
            >
              <option value="splendid">🎹 Splendid</option>
              <option value="salamander">🎸 Salamander</option>
            </select>
            <div class="absolute right-3 top-1/2 transform -translate-y-1/2 pointer-events-none">
              <svg class="w-4 h-4 text-zinc-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </div>
          </div>
          
          {/* Instrument indicator */}
          <div class="flex items-center gap-2 px-3 py-1.5 bg-zinc-800/50 rounded-lg border border-zinc-700/30">
            <div class={`w-2 h-2 rounded-full ${selectedInstrument() === 'splendid' ? 'bg-emerald-500' : 'bg-purple-500'} animate-pulse`}></div>
            <span class="text-xs text-zinc-400 font-medium uppercase tracking-wide">
              {selectedInstrument() === 'splendid' ? 'Grand Piano' : 'Stage Piano'}
            </span>
          </div>
        </div>
        
        {/* Velocity Selector */}
        <VelocitySelector
          selectedLayer={selectedLayer}
          onLayerChange={onLayerChange}
          onLayerHover={onLayerHover}
          layers={layers}
          instrumentName={selectedInstrument}
        />
      </div>
      
      <VolumeControl />
    </div>
  );
}