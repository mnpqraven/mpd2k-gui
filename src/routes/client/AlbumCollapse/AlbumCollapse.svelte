<script lang="ts">
  import * as Collapsible from "$lib/components/ui/collapsible";
  import type { AlbumIpc, AudioTrackIpc } from "../../../bindings/taurpc";
  import { Button } from "$lib/components/ui/button";
  import { rpc } from "$lib/ipc";
  import { getClientViewStore } from "$lib/state/clientView.svelte";
  export let open: boolean;
  export let album: AlbumIpc;

  const clientView = getClientViewStore();
  function onSelect(_: boolean) {
    if (album.meta.name) {
      clientView.selectAlbum(album.meta.name);
    }
  }

  const rows = Math.ceil(album.tracks.length / 2);
  async function onPlay(track: AudioTrackIpc) {
    console.log("playing", track);
    const r = await rpc();
    await r.playback.play(track);
  }
</script>

<Collapsible.Root {open} onOpenChange={onSelect} class="flex">
  <Collapsible.Trigger class="h-16 w-16 bg-red-400"></Collapsible.Trigger>
  <Collapsible.Content
    class={`grid grid-flow-col grid-rows-6 gap-2`}
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
        <div>1:00</div>
      </Button>
    {/each}
  </Collapsible.Content>
</Collapsible.Root>
