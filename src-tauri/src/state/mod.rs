use crate::config::DotfileSchema;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub dot_config: DotfileSchema,
}

#[derive(Clone)]
pub struct AppStateArc(pub Arc<Mutex<AppState>>);

impl Default for AppStateArc {
    fn default() -> Self {
        Self::new()
    }
}

impl AppStateArc {
    pub fn new() -> Self {
        let state = AppState {
            dot_config: DotfileSchema::parse().unwrap(),
        };
        Self(Arc::new(Mutex::new(state)))
    }
    pub fn arced(&self) -> Arc<Mutex<AppState>> {
        self.0.clone()
    }
}
