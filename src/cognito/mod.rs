pub mod handlers;
pub mod models;
pub mod state;

use axum::{routing::post, Router};

use crate::memory::store::SharedStore;

pub fn routes(store: SharedStore) -> Router {
    Router::new()
        .route(
            "/cognito/admin_initiate_auth",
            post(handlers::admin_initiate_auth),
        )
        .route("/cognito/signup", post(handlers::signup)) // Add the new signup route
        .with_state(store)
}
