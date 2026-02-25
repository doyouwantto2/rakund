import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";
import { ErrorBoundary } from "solid-js";
import Home from "./pages/Home";
import '@/assets/styles/global.css';

render(
  () => (
    <ErrorBoundary fallback={(err) => (
      <div class="h-screen w-full bg-zinc-950 text-red-400 flex flex-col items-center justify-center gap-4 p-8">
        <p class="text-lg font-bold">Something went wrong</p>
        <pre class="text-xs bg-zinc-900 p-4 rounded max-w-2xl overflow-auto">
          {err?.message ?? String(err)}
        </pre>
        <button
          class="px-4 py-2 bg-zinc-800 rounded text-zinc-200 text-sm"
          onClick={() => location.reload()}
        >
          Reload
        </button>
      </div>
    )}>
      <Router>
        <Route path="/" component={Home} />
      </Router>
    </ErrorBoundary>
  ),
  document.getElementById("root") as HTMLElement
);
