<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { getAppStore } from "$lib/state/appStore.svelte";
  import { getClientViewStore } from "$lib/state/clientView.svelte";
  import AlbumCollapse from "./AlbumCollapse/AlbumCollapse.svelte";
  import { PanelList } from "./PanelList";

  const appstore = getAppStore();
  const clientView = getClientViewStore();
  const libTree = $derived(appstore.libTree);

  let albumArtists = $derived(
    Array.from(
      new Set(
        appstore.libTree
          .map((e) => e.meta.album_artist)
          .filter((e) => e !== null) as string[],
      ),
    ),
  );
</script>

<div class="flex h-full justify-between">
  <PanelList {albumArtists} />

  <div class="flex flex-1 flex-col gap-4">
    {#each libTree as album (album["meta"]["name"])}
      <AlbumCollapse
        open={clientView.selectedAlbum === album.meta.name}
        {album}
      />
    {/each}
  </div>

  <div class="w-60 rounded-md border">right</div>
</div>
