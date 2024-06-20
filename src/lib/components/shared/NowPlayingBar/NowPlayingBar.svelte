<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import {
    ContextMenu,
    ContextMenuTrigger,
  } from "$lib/components/ui/context-menu";
  import ContextMenuContent from "$lib/components/ui/context-menu/context-menu-content.svelte";
  import { Slider } from "$lib/components/ui/slider";
  import { Toggle } from "$lib/components/ui/toggle";
  import { rpc } from "$lib/ipc";
  import { getPlaybackStore } from "$lib/state/playback.svelte";
  import { cn } from "$lib/utils";
  import {
    Pause,
    Play,
    Repeat,
    Repeat1,
    Shuffle,
    SkipBack,
    SkipForward,
    Volume2,
  } from "lucide-svelte";
  import { onDestroy } from "svelte";

  let playbackState = getPlaybackStore();

  let lastTime = window.performance.now();

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

<div class="flex border-t">
  <Button variant="outline" class="p-2">
    <SkipBack />
  </Button>

  <Button
    on:click={() => playbackState.play_pause()}
    variant="outline"
    class="p-2"
  >
    {#if playbackState.status === "Play"}
      <Pause />
    {:else}
      <Play />
    {/if}
  </Button>

  <Button variant="outline" class="p-2">
    <SkipForward />
  </Button>

  <Toggle
    class={cn("p-2")}
    pressed={playbackState.shuffle}
    onPressedChange={async (e) => {
      await playbackState.setShuffle(e);
    }}
  >
    <Shuffle />
  </Toggle>
  <Button
    variant="outline"
    class="p-2"
    on:click={async () => {
      void playbackState.cycleRepeat();
    }}
  >
    {#if playbackState.repeat === "Off"}
      <Repeat class="text-muted" />
    {:else if playbackState.repeat === "Repeat"}
      <Repeat />
    {:else}
      <Repeat1 />
    {/if}
  </Button>

  {prettyPrintSecs(playbackState.elapsedDuration)}

  <ContextMenu>
    <ContextMenuTrigger>
      <Button variant="outline">
        <Volume2 />
      </Button>
    </ContextMenuTrigger>
    <ContextMenuContent class="flex w-48 gap-4 p-4">
      <Slider
        max={100}
        step={1}
        bind:value={playbackState.volume}
        onValueChange={async (a) => {
          // better to put in a mini debounce
          const r = await rpc();
          r.playback.set_volume(a[0]);
        }}
      />

      <span class="w-8">
        {playbackState.volume[0]}
      </span>
    </ContextMenuContent>
  </ContextMenu>
</div>
