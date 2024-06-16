pub mod playback;
pub mod root;

use std::sync::Arc;

use crate::{error::AppError, state::AppStateArc};
use playback::{PlaybackApi, PlaybackState};
use rodio::Sink;
use root::RootApi;
use taurpc::Router;

pub async fn create_router(state: AppStateArc, sink: Arc<Sink>) -> Result<Router, AppError> {
    let router = Router::new()
        .merge(AppStateArc::into_handler(state))
        .merge(PlaybackState::into_handler(PlaybackState::new(sink)));
    Ok(router)
}
