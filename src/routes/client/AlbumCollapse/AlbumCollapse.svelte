<script lang="ts">
  import * as Collapsible from "$lib/components/ui/collapsible";
  import type { AlbumIpc, AudioTrackIpc } from "../../../bindings/taurpc";
  import { Button } from "$lib/components/ui/button";
  import { rpc } from "$lib/ipc";
  import { getClientViewStore } from "$lib/state/clientView.svelte";
  import { getPlaybackStore } from "$lib/state/playback.svelte";

  export let album: AlbumIpc;

  let open: boolean = true;
  const rows = Math.ceil(album.tracks.length / 2);
  const clientView = getClientViewStore();
  let playbackState = getPlaybackStore();

  function onSelect(to: boolean) {
    open = to;
    if (album.meta.name) {
      clientView.selectAlbum(album.meta.name);
    }
  }

  async function onPlay(track: AudioTrackIpc) {
    const r = await rpc();
    await r.playback.play(track);
    playbackState.elapsedDuration = 0;
  }
</script>

<Collapsible.Root {open} onOpenChange={onSelect}>
  <Collapsible.Trigger class="flex gap-2">
    <div class="h-16 w-16 bg-red-400"></div>
    <span>{album.meta.name}</span>
  </Collapsible.Trigger>
  <Collapsible.Content
    class={`grid grid-flow-col gap-2 mt-4`}
    style={`grid-template-rows: repeat(${rows}, minmax(0, 1fr))`}
  >
    {#each album.tracks as track (track["path"])}
      <Button
        class="flex gap-2"
        ondblclick={() => onPlay(track)}
        variant="outline"
      >
        <div>{track.track_no}</div>
        <div class="flex flex-1 flex-col text-left">
          <div>{track.name}</div>
          <div>FLAC</div>
        </div>
        <div>01:00</div>
      </Button>
    {/each}
  </Collapsible.Content>
</Collapsible.Root>
