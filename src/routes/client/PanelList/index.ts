import type { AlbumIpc } from "../../../bindings/taurpc";
import PanelList from "./PanelList.svelte";

interface Props {
  albumArtists: string[];
}

const list: AlbumIpc[] = [];
const wtf = list.map((e) => e.meta.album_artist).filter((e) => e !== null);

export { PanelList, type Props };
