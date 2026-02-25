import { Match, Switch } from "solid-js";
import type { Accessor } from "solid-js";
import RainLayout from "./rain/RainLayout";
// SheetLayout will be imported here when sheet scene is implemented
import type { MidiNoteMs, SessionMode, SessionStatus } from "@/hooks/useBuffer";

interface SceneDispatcherProps {
  allNotes: Accessor<MidiNoteMs[]>;
  currentTime: Accessor<number>;
  sessionStatus: Accessor<SessionStatus>;
  sessionMode: Accessor<SessionMode>;
}

export default function SceneDispatcher(props: SceneDispatcherProps) {
  return (
    <div class="flex-1 relative overflow-hidden">
      <Switch>
        {/* Blank — no session active */}
        <Match when={props.sessionStatus() === "idle"}>
          <div class="w-full h-full flex items-center justify-center">
            <span class="text-zinc-700 text-sm select-none">
              Select an instrument and a song to begin
            </span>
          </div>
        </Match>

        {/* Rain — session is ready/playing/paused/finished */}
        <Match
          when={
            (props.sessionStatus() === "ready" ||
              props.sessionStatus() === "playing" ||
              props.sessionStatus() === "paused" ||
              props.sessionStatus() === "finished") &&
            props.sessionMode() !== null
          }
        >
          <RainLayout
            allNotes={props.allNotes}
            currentTime={props.currentTime}
            sessionMode={props.sessionMode}
          />
        </Match>

        {/* Ready but no mode selected yet — show blank with a subtle hint */}
        <Match when={props.sessionStatus() === "ready"}>
          <div class="w-full h-full flex items-center justify-center">
            <span class="text-zinc-700 text-sm select-none">
              Select a mode to begin
            </span>
          </div>
        </Match>
      </Switch>
    </div>
  );
}
