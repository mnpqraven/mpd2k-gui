use crate::{error::AppError, load::load_tree, meta::types::AlbumIpc, state::AppStateArc};
use tauri::AppHandle;

// Root procedures
#[taurpc::procedures(event_trigger = RootApiEvent, export_to = "../src/bindings/taurpc.ts")]
pub trait RootApi {
    async fn init(app_handle: AppHandle<tauri::Wry>);

    #[taurpc(event)]
    async fn load_music(data: Vec<AlbumIpc>);
}

#[taurpc::resolvers]
impl RootApi for AppStateArc {
    async fn init(self, app_handle: AppHandle<tauri::Wry>) {
        println!("loading music");

        let inner = self.arced();

        // TODO: task for loading cache

        tokio::spawn(async move {
            let guard = inner.lock().await;
            let trigger = RootApiEvent::new(app_handle);
            let conf = guard.dot_config.clone();
            drop(guard);

            // generate the whole lib tree
            let root_path = conf.library_root()?;
            let tree = load_tree(root_path).await?;

            // serialize tree to frontend
            let tree: Vec<AlbumIpc> = tree
                .lock()
                .await
                .clone()
                .into_iter()
                .map(|(meta, tracks)| AlbumIpc {
                    meta: meta.into(),
                    tracks: tracks.into_iter().map(|e| e.into()).collect(),
                })
                .collect();
            trigger.load_music(tree).unwrap();
            Ok::<(), AppError>(())
        });
    }
}
