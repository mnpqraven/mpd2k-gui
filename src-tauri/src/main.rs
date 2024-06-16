// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::incompatible_msrv)]

use constants::HASH_CONCURRENT_LIMIT;
use error::AppError;
use playback::PlaybackServer;
use router::create_router;
use state::AppStateArc;
use tauri::Manager;
use tokio::runtime::Handle;

pub mod config;
pub mod constants;
pub mod error;
pub mod load;
pub mod meta;
pub mod playback;
pub mod router;
pub mod state;

fn main() -> Result<(), AppError> {
    // NOTE: clap arg here if needed

    let hashing_rt = tokio::runtime::Builder::new_multi_thread()
        .thread_name("hashing thread")
        .worker_threads(HASH_CONCURRENT_LIMIT)
        .enable_all()
        .build()
        .unwrap();
    let hashing_handle = hashing_rt.handle();

    let main_rt = tokio::runtime::Builder::new_multi_thread()
        .thread_name("main thread")
        .enable_all()
        .build()
        .unwrap();

    main_rt.block_on(_main(hashing_handle))?;

    Ok(())
}

async fn _main(_hash_handle: &Handle) -> Result<(), AppError> {
    // TODO: unwrap
    let playback_server = PlaybackServer::new();
    let app = AppStateArc::new();

    let router = create_router(app, playback_server.handle()).await.unwrap();

    tauri::Builder::default()
        .invoke_handler(router.into_handler())
        .setup(|app| {
            #[cfg(debug_assertions)]
            if let Some(window) = app.get_window("main") {
                window.open_devtools();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
