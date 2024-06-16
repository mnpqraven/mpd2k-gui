import { setContext, getContext } from "svelte";
import { CONTEXT } from ".";

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
  const state = new ClientViewStore({
    selectedAlbumArtist: undefined,
    selectedAlbum: undefined,
  });
  setContext(CONTEXT.clientView, state);
  return state;
}
export function getClientViewStore() {
  return getContext<ClientViewStore>(CONTEXT.clientView);
}
