<script lang="ts">
  import { ScrollArea } from "$lib/components/ui/scroll-area";
  import { getAppStore } from "$lib/state/appStore.svelte";
  import AlbumCollapse from "./AlbumCollapse/AlbumCollapse.svelte";
  import { PanelList } from "./PanelList";

  const appstore = getAppStore();
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

  <ScrollArea class="h-[calc(100vh-100px)] w-full">
    <div class="flex flex-col gap-4">
      {#each libTree as album (album["meta"]["name"])}
        <AlbumCollapse {album} />
      {/each}
    </div>
  </ScrollArea>

  <div class="w-60 rounded-md border">right</div>
</div>
