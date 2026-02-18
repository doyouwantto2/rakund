interface LoadingIndicatorProps {
  isLoading: boolean;
  instrument: string;
  layer: string;
  progress?: { loaded: number; total: number; progress: number } | null;
}

export default function LoadingIndicator(props: LoadingIndicatorProps) {
  const { isLoading, instrument, layer, progress } = props;
  
  if (!isLoading) return null;
  
  return (
    <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-black/90 backdrop-blur-md text-white px-6 py-4 rounded-2xl shadow-2xl border border-zinc-700/50 z-50">
      <div class="flex items-center gap-4">
        {/* Animated loading icon */}
        <div class="relative">
          <div class="w-8 h-8 border-2 border-zinc-600 border-t-emerald-500 rounded-full animate-spin"></div>
          <div class="absolute inset-0 w-8 h-8 border-2 border-transparent border-t-purple-500 rounded-full animate-spin animation-delay-150"></div>
        </div>
        
        <div>
          <div class="text-sm font-bold text-zinc-100">
            {progress ? `Loading Samples (${progress.loaded}/${progress.total})` : 'Loading Samples'}
          </div>
          <div class="flex items-center gap-2 text-xs text-zinc-400">
            <span class={`px-2 py-0.5 rounded-full ${
              instrument === 'splendid' 
                ? 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30' 
                : 'bg-purple-500/20 text-purple-400 border border-purple-500/30'
            }`}>
              {instrument === 'splendid' ? '🎹' : '🎸'} {instrument}
            </span>
            <span class="text-zinc-500">→</span>
            <span class="px-2 py-0.5 bg-zinc-700/50 rounded-full text-zinc-300">
              {layer}
            </span>
          </div>
        </div>
      </div>
      
      {/* Progress bar */}
      <div class="mt-3 w-full bg-zinc-800/50 rounded-full h-1 overflow-hidden">
        <div class="h-full bg-gradient-to-r from-emerald-500 to-purple-500 rounded-full transition-all duration-300 ease-out" 
             style={{
               width: progress ? `${progress.progress}%` : "60%"
             }}>
        </div>
      </div>
      
      {/* Progress percentage */}
      {progress && (
        <div class="mt-1 text-xs text-zinc-400 text-center">
          {Math.round(progress.progress)}%
        </div>
      )}
    </div>
  );
}
