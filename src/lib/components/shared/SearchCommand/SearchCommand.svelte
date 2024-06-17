<script lang="ts">
  import * as Command from "$lib/components/ui/command";
  import { createKeybind } from "$lib/creates/keybind";
  import { getAppStore } from "$lib/state/appStore.svelte";
  import { onMount } from "svelte";
  import type { AudioTrackIpc } from "../../../../bindings/taurpc";
  import { rpc } from "$lib/ipc";
  import { getPlaybackStore } from "$lib/state/playback.svelte";

  let libtree = getAppStore();
  let playbackState = getPlaybackStore();

  const { handleKeyDown } = createKeybind({
    key: "f",
    modifiers: ["ctrl"],
    action: () => {
      libtree.searchOpen = !libtree.searchOpen;
    },
  });

  onMount(() => {
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  });

  async function onSelect(track: AudioTrackIpc) {
    libtree.searchOpen = false;
    void playbackState.play(track);
  }
</script>

<Command.Dialog bind:open={libtree.searchOpen} loop>
  <Command.Input placeholder="Type a command or search..." />
  <Command.List>
    <Command.Empty>No results found.</Command.Empty>
    {#each libtree.libTree as album (album.meta.name)}
      <Command.Group heading={album.meta.name ?? ""}>
        {#each album.tracks as track}
          <Command.Item onSelect={() => onSelect(track)}
            >{track.name}</Command.Item
          >
        {/each}
      </Command.Group>
    {/each}
  </Command.List>
</Command.Dialog>
