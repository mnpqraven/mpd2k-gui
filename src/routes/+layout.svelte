<script lang="ts">
  import { ModeWatcher } from "mode-watcher";
  import "../app.css";
  import { AppTopbar } from "$lib/components/shared/AppTopbar";
  import { ThemeSwitcher } from "$lib/components/shared/ThemeSwitcher";
  import { NowPlayingBar } from "$lib/components/shared/NowPlayingBar";
  import { rpc } from "$lib/ipc";
  import { onDestroy, onMount } from "svelte";
  import { setAppStore, setClientViewStore } from "$lib/state.svelte";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  let unlisten: UnlistenFn;

  let libtree = setAppStore();
  setClientViewStore();

  onMount(async () => {
    (await rpc()).load_all();
    unlisten = (await rpc()).load_music.on((e) => {
      console.log("yo from frontend", e);
      // update stuff from init
      libtree.libTree = [...e];
    });
  });

  onDestroy(() => {
    unlisten();
  });
</script>

<div class="flex h-screen w-screen flex-col">
  <div class="flex justify-between gap-1 p-1">
    <AppTopbar />
    <ThemeSwitcher />
  </div>

  <div class="flex-1 grow">
    <slot></slot>
  </div>

  <div>tree length: {libtree.libTree.length}</div>

  <NowPlayingBar />
</div>

<ModeWatcher />
