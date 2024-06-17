<script lang="ts">
  import { ModeWatcher } from "mode-watcher";
  import "../app.css";
  import { AppTopbar } from "$lib/components/shared/AppTopbar";
  import { ThemeSwitcher } from "$lib/components/shared/ThemeSwitcher";
  import { NowPlayingBar } from "$lib/components/shared/NowPlayingBar";
  import { rpc } from "$lib/ipc";
  import { onDestroy, onMount } from "svelte";
  import { setAppStore } from "$lib/state/appStore.svelte";
  import type { UnlistenFn } from "@tauri-apps/api/event";
  import { setClientViewStore } from "$lib/state/clientView.svelte";
  import { setPlaybackStore } from "$lib/state/playback.svelte";
  import SearchCommand from "$lib/components/shared/SearchCommand/SearchCommand.svelte";

  let unlisten: UnlistenFn[] = [];

  let libtree = setAppStore();
  setClientViewStore();
  let playbackState = setPlaybackStore();

  onMount(async () => {
    let r = await rpc();
    r.load_all();
    let fn_lib = (await rpc()).load_music.on((e) => {
      console.log("[event] load_music", e);
      // update stuff from init
      libtree.libTree = [...e];
    });

    let fn_playback = r.playback.ev_playback_state.on((e) => {
      console.log("[event] ev_playback_state", e);
      playbackState.updateFromState(e);
    });

    unlisten = [...unlisten, fn_lib, fn_playback];
  });

  onDestroy(() => {
    unlisten.forEach((fn) => fn());
  });
</script>

<div class="flex h-screen w-screen flex-col">
  <div class="flex justify-between gap-1 p-1">
    <AppTopbar />
    <ThemeSwitcher />
  </div>

  <div class="flex-1 grow" id="main-container">
    <slot></slot>
  </div>

  <NowPlayingBar />
</div>

<SearchCommand />

<ModeWatcher />
