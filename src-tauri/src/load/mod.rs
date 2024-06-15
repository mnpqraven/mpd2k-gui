use crate::{
    error::AppError,
    meta::{
        is_supported_audio,
        types::{AlbumMeta, AudioTrack},
    },
};
use std::{collections::BTreeMap, fmt::Debug, path::Path, sync::Arc};
use tokio::{sync::Mutex, task::JoinSet};
use tracing::{error, instrument};
use walkdir::WalkDir;

pub type AudioTreeArc = Arc<Mutex<BTreeMap<AlbumMeta, Vec<AudioTrack>>>>;

#[instrument]
pub async fn load_tree<P: AsRef<Path> + Debug>(
    lib_root: P,
    // lib_arc: Arc<Mutex<LibraryClient>>,
    // TODO: impl hard_update
) -> Result<AudioTreeArc, AppError> {
    let library_tree = WalkDir::new(lib_root).follow_links(true);

    let tree: AudioTreeArc = Arc::new(Mutex::new(BTreeMap::new()));

    let mut tagging_task = JoinSet::new();

    for entry in library_tree {
        let entry = entry?;

        let path = entry.path().to_string_lossy().to_string();
        let tree_arc = tree.clone();
        if is_supported_audio(&path) {
            tagging_task.spawn(async move {
                let mut track = AudioTrack::new(path);
                track.update_tag()?;

                let track_meta = AlbumMeta::from(&track);

                // add to album BTree
                let mut tree = tree_arc.lock().await;
                match tree.get_mut(&track_meta) {
                    Some(val) => {
                        val.push(track.clone());
                        val.sort_unstable();
                    }
                    None => {
                        tree.insert(track_meta, vec![track.clone()]);
                    }
                }

                Ok::<AudioTrack, AppError>(track)
            });
        }
    }

    while let Some(Ok(trk)) = tagging_task.join_next().await {
        if let Err(e) = trk {
            error!(?e);
        }
    }

    Ok(tree)
}
