interface LoadingProgressProps {
  progress: number | null;
  instrumentName: string;
}

export default function LoadingProgress(props: LoadingProgressProps) {
  const progressPct = () => {
    return props.progress !== null ? Math.max(0, Math.min(100, props.progress)) : 0;
  };

  return (
    <div class="bg-zinc-800 rounded-lg p-2 shadow-lg border border-zinc-700">
      <div class="text-xs text-zinc-400 mb-1">Loading {props.instrumentName}...</div>
      <div class="w-48 bg-zinc-700 rounded-full h-2 overflow-hidden">
        <div 
          class="h-full bg-amber-400 transition-all duration-300 ease-out"
          style={{ width: `${progressPct()}%` }}
        />
      </div>
      <div class="text-xs text-zinc-400 mt-1 text-center">
        {progressPct().toFixed(1)}%
      </div>
    </div>
  );
}
