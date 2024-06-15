import { getContext, setContext } from "svelte";
import type { AlbumIpc, AudioTrackIpc } from "../bindings/taurpc";

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

const CONTEXT = {
  appStore: "appStore",
  libraryTree: "libraryTree",
  clientView: "clientView",
};

export function setAppStore(initialData?: AppStoreArgs) {
  const libraryTreeState = new AppStore(initialData);
  setContext(CONTEXT.appStore, libraryTreeState);
  return libraryTreeState;
}

export function getAppStore() {
  return getContext<AppStore>(CONTEXT.appStore);
}

interface ClientViewStoreArgs {
  selectedAlbumArtist: string | undefined;
  selectedAlbum: string | undefined;
}
class ClientViewStore {
  selectedAlbumArtist: string | undefined = $state(undefined);
  selectedAlbum: string | undefined = $state(undefined);

  constructor(initialData: ClientViewStoreArgs) {
    this.selectedAlbumArtist = initialData.selectedAlbumArtist;
    this.selectedAlbum = initialData.selectedAlbum;
  }
  selectAlbumArtist(to: string) {
    this.selectedAlbumArtist = to;
  }
  selectAllAlbumArtist() {
    this.selectedAlbumArtist = undefined;
  }
  selectAlbum(to: string) {
    if (to !== this.selectedAlbum) this.selectedAlbum = to;
    else this.selectedAlbum = undefined;
  }
}
export function setClientViewStore() {
  const clientViewState = new ClientViewStore({
    selectedAlbumArtist: undefined,
    selectedAlbum: undefined,
  });
  setContext(CONTEXT.clientView, clientViewState);
  return clientViewState;
}
export function getClientViewStore() {
  return getContext<ClientViewStore>(CONTEXT.clientView);
}
