<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { rpc } from "$lib/ipc";
  import { getPlaybackStore } from "$lib/state/playback.svelte";
  import {
    Pause,
    Play,
    Repeat,
    Repeat1,
    Shuffle,
    SkipBack,
    SkipForward,
  } from "lucide-svelte";
  import { onDestroy } from "svelte";

  let playbackState = getPlaybackStore();

  let lastTime = window.performance.now();
  // duration of track

  let frame: number;
  (function update() {
    frame = requestAnimationFrame(update);

    const time = window.performance.now();
    if (playbackState.status === "Play") {
      playbackState.elapsedDuration += time - lastTime;
    }
    lastTime = time;
  })();

  onDestroy(() => {
    cancelAnimationFrame(frame);
  });

  async function onPlayToggle() {
    const r = await rpc();
    if (playbackState.status === "Play") {
      // resumes timer
      console.log("playing");
    } else {
      // pause timer
      console.log("pausing");
    }
    await r.playback.pause_toggle();
  }

  function prettyPrintSecs(ms: number): string {
    const secs = ms / 1000;
    const mins = Math.floor(secs / 60);
    const restSecs = secs % 60;
    const hours = Math.floor(mins / 60);
    const restMins = mins % hours;
    if (hours === 0)
      return `${mins.toFixed(0)}:${restSecs.toFixed(0).padStart(2, "0")}`;
    return `${hours.toFixed(0)}:${restMins.toFixed(0)}:${restSecs.toFixed(0).padStart(2, "0")}`;
  }
</script>

<div class="border-t">
  <Button variant="outline" class="p-2">
    <SkipBack />
  </Button>

  <Button on:click={onPlayToggle} variant="outline" class="p-2">
    {#if playbackState.status === "Play"}
      <Pause />
    {:else}
      <Play />
    {/if}
  </Button>

  <Button variant="outline" class="p-2">
    <SkipForward />
  </Button>

  <Button>
    <Shuffle />
  </Button>
  <Button>
    <Repeat />
  </Button>
  <Button>
    <Repeat1 />
  </Button>

  {prettyPrintSecs(playbackState.elapsedDuration)}
</div>
