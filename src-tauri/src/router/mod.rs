use crate::{
    error::AppError,
    state::{Api, AppStateArc, Events, EventsImpl},
};
use taurpc::Router;

pub async fn create_router() -> Result<Router, AppError> {
    let router = Router::new()
        .merge(AppStateArc.into_handler())
        .merge(EventsImpl.into_handler());
    Ok(router)
}
