pub mod playback;
pub mod root;

use std::sync::Arc;

use crate::{error::AppError, state::AppStateArc};
use playback::{PlaybackApi, PlaybackState};
use rodio::Sink;
use root::RootApi;
use taurpc::Router;

use self::playback::PlaybackStateImpl;

pub async fn create_router(state: AppStateArc, sink: Arc<Sink>) -> Result<Router, AppError> {
    let playback_state = PlaybackState::new(sink);

    let router = Router::new().merge(AppStateArc::into_handler(state)).merge(
        PlaybackStateImpl::into_handler(PlaybackStateImpl::new(playback_state)),
    );
    Ok(router)
}
