import { setContext, getContext } from "svelte";
import { CONTEXT } from ".";
import type { PlaybackState } from "../../bindings/taurpc";
import { writable, type Writable } from "svelte/store";

class PlaybackStore {
  nowPlaying: PlaybackState["now_playing"] = $state(null);
  elapsedDuration: number = $state(0);
  duration: PlaybackState["duration_secs"] = $state(null);
  status: PlaybackState["status"] = $state("Stopped");

  updateFromState({ duration_secs, now_playing, status }: PlaybackState) {
    this.duration = duration_secs;
    if (this.nowPlaying?.name !== this.nowPlaying?.name) {
      this.nowPlaying = now_playing;
      this.elapsedDuration = 0;
    }

    if (this.status !== status)
      switch (status) {
        // TODO: will have a HardPlay enum later
        case "Play":
        case "Paused":
        case "Stopped": {
          this.elapsedDuration = 0;
        }
        default:
          this.status = status;
      }
  }
}
export function setPlaybackStore() {
  const state = new PlaybackStore();
  setContext(CONTEXT.playback, state);
  return state;
}

export function getPlaybackStore() {
  return getContext<PlaybackStore>(CONTEXT.playback);
}
