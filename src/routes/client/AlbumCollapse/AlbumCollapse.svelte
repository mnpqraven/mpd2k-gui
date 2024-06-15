<script lang="ts">
  import * as Collapsible from "$lib/components/ui/collapsible";
  import type { AlbumIpc } from "../../../bindings/taurpc";
  import { getClientViewStore } from "$lib/state.svelte";
  export let open: boolean;
  export let album: AlbumIpc;

  const clientView = getClientViewStore();
  function onSelect(_: boolean) {
    if (album.meta.name) {
      clientView.selectAlbum(album.meta.name);
    }
  }

  const rows = Math.ceil(album.tracks.length / 2);
  console.log(rows);
</script>

<Collapsible.Root {open} onOpenChange={onSelect} class="flex">
  <Collapsible.Trigger class="h-16 w-16 bg-red-400"></Collapsible.Trigger>
  <Collapsible.Content
    class={`grid grid-flow-col grid-rows-6 gap-2`}
    style={`grid-template-rows: repeat(${rows}, minmax(0, 1fr))`}
  >
    {#each album.tracks as track (track["path"])}
      <div class="flex gap-2">
        <div>{track.track_no}</div>
        <div class="flex flex-1 flex-col text-left">
          <div>{track.name}</div>
          <div>FLAC</div>
        </div>
        <div>1:00</div>
      </div>
    {/each}
  </Collapsible.Content>
</Collapsible.Root>
