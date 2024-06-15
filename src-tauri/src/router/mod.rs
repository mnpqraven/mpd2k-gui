pub mod root;

use crate::{error::AppError, state::AppStateArc};
use root::RootApi;
use taurpc::Router;

pub async fn create_router(state: AppStateArc) -> Result<Router, AppError> {
    let router = Router::new().merge(AppStateArc::into_handler(state));
    Ok(router)
}
