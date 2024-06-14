use std::sync::Arc;
use tokio::sync::Mutex;

// Root procedures
#[taurpc::procedures]
pub trait Api {
    async fn hello_world();
}

#[derive(Clone)]
pub struct AppState {}

#[derive(Clone)]
pub struct AppStateArc(Arc<Mutex<AppState>>);

#[taurpc::resolvers]
impl Api for AppStateArc {
    async fn hello_world(self) {
        println!("Hello world");
    }
}

// Nested procedures, you can also do this (path = "api.events.users")
#[taurpc::procedures(path = "events")]
pub trait Events {
    #[taurpc(event)]
    async fn event();
}

#[derive(Clone)]
pub struct EventsImpl;

#[taurpc::resolvers]
impl Events for EventsImpl {}
