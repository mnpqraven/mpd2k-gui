// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::incompatible_msrv)]

use router::create_router;

pub mod error;
pub mod router;
pub mod state;

#[tokio::main]
async fn main() {
    // TODO: unwrap
    let router = create_router().await.unwrap();

    tauri::Builder::default()
        .invoke_handler(router.into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
