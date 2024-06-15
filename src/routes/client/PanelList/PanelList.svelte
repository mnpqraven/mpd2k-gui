<script lang="ts">
  import { cn } from "$lib/utils";
  import type { HTMLAttributes } from "svelte/elements";
  import { type Props } from "./index";
  import { Button } from "$lib/components/ui/button";
  import { getClientViewStore } from "$lib/state.svelte";

  type $$Props = HTMLAttributes<HTMLDivElement> & Props;

  let className: $$Props["class"] = undefined;
  export let albumArtists: Props["albumArtists"] = [];
  export { className as class };

  let clientView = getClientViewStore();

  function onSelectAA(val: string | undefined) {
    if (!val) clientView.selectAllAlbumArtist();
    else clientView.selectAlbumArtist(val);
  }
</script>

<div
  class={cn(className, "flex flex-col gap-2 rounded-md border")}
  {...$$restProps}
>
  <Button variant="ghost" size="sm" on:click={() => onSelectAA(undefined)}>
    All
  </Button>

  {#each albumArtists as albumArtist}
    <Button variant="ghost" size="sm" on:click={() => onSelectAA(albumArtist)}>
      {albumArtist}
    </Button>
  {/each}
</div>
