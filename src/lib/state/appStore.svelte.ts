import { getContext, setContext } from "svelte";
import type { AlbumIpc, AudioTrackIpc } from "../../bindings/taurpc";
import { CONTEXT } from ".";

interface AppStoreArgs {
  libTree: AlbumIpc[];
  nowPlaying: AudioTrackIpc | undefined;
}
const defaultAppStore: AppStoreArgs = {
  libTree: [],
  nowPlaying: undefined,
};
class AppStore {
  libTree: AlbumIpc[] = $state([]);
  nowPlaying: AudioTrackIpc | undefined = $state(undefined);

  constructor(initialData: AppStoreArgs = defaultAppStore) {
    this.libTree = initialData.libTree;
  }
}

export function setAppStore(initialData?: AppStoreArgs) {
  const state = new AppStore(initialData);
  setContext(CONTEXT.appStore, state);
  return state;
}

export function getAppStore() {
  return getContext<AppStore>(CONTEXT.appStore);
}
