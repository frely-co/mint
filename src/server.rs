use axum::Router;
use crate::cognito;
use crate::memory::store::{SharedStore, MemoryStore};

pub fn create_router(store: SharedStore) -> Router {
    Router::new()
        .merge(cognito::routes(store))
        // Add other modules like S3 here
}

