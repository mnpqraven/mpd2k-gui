import { setContext, getContext } from "svelte";
import { CONTEXT } from ".";
import type { AudioTrackIpc, PlaybackState } from "../../bindings/taurpc";
import { rpc } from "$lib/ipc";

class PlaybackStore {
  nowPlaying: PlaybackState["now_playing"] = $state(null);
  elapsedDuration: number = $state(0);
  duration: PlaybackState["duration_secs"] = $state(null);
  status: PlaybackState["status"] = $state("Stopped");

  shuffle: boolean = $state(false);
  repeat: PlaybackState["repeat"] = $state("Off");

  constructor() {
    // this.shuffle = false;
  }

  updateFromState({ duration_secs, now_playing, status }: PlaybackState) {
    this.duration = duration_secs;
    if (this.nowPlaying?.name !== this.nowPlaying?.name) {
      this.nowPlaying = now_playing;
      this.elapsedDuration = 0;
    }

    if (this.status !== status) {
      switch (status) {
        // TODO: will have a HardPlay enum later
        case "Play":
          break;
        case "Pause":
          break;
        case "Stopped": {
          this.elapsedDuration = 0;
          break;
        }
      }
      this.status = status;
    }
  }

  async setShuffle(to: boolean) {
    const r = await rpc();
    // toggle state
    this.shuffle = await r.playback.set_shuffle(to);
  }

  async cycleRepeat() {
    const r = await rpc();
    this.repeat = await r.playback.cycle_repeat();
  }

  async play(track: AudioTrackIpc) {
    const r = await rpc();
    await r.playback.play(track);
    this.elapsedDuration = 0;
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
