import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [status, setStatus] = createSignal("Ready to play");

  // Logic to trigger the Rust command
  const playNote = async (noteName: string) => {
    try {
      setStatus(`Playing ${noteName}...`);

      // 'path' must match your Rust function parameter name exactly
      await invoke("play_note", {
        path: `../src-tauri/data/splendid/Samples/${noteName}.flac`
      });

    } catch (error) {
      console.error("Audio Error:", error);
      setStatus("Error: Check console");
    }
  };

  return (
    <div style={{ padding: "20px", "text-align": "center" }}>
      <h1>SolidJS Piano</h1>
      <p>Status: {status()}</p>

      <div style={{ display: "flex", gap: "10px", "justify-content": "center" }}>
        {/* Basic Buttons */}
        <button
          class="piano-key"
          onClick={() => playNote("PP C#5")}
        >
          C#5
        </button>

        <button
          class="piano-key"
          onClick={() => playNote("Mp B5")}
        >
          B5
        </button>

        {/* Simultaneous Play (Polyphony) */}
        <button
          style={{ "background-color": "#4caf50", color: "white" }}
          onClick={() => {
            playNote("PP C#5");
            playNote("Mp B5");
          }}
        >
          Play Both (Chord)
        </button>
      </div>
    </div>
  );
}

export default App;
