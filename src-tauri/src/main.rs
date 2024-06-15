// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::incompatible_msrv)]

use router::create_router;
use state::AppStateArc;
use tauri::Manager;

pub mod config;
pub mod error;
pub mod load;
pub mod meta;
pub mod router;
pub mod state;

#[tokio::main]
async fn main() {
    // TODO: unwrap
    let app = AppStateArc::new();

    let router = create_router(app).await.unwrap();

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
}
