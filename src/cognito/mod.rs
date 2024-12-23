// src/cognito/mod.rs

pub mod handlers;
pub mod models;
pub mod state;

use axum::{routing::post, Router};

use crate::cognito::handlers::admin_initiate_auth;

use crate::memory::store::SharedStore;

pub fn routes(store: SharedStore) -> Router {
    Router::new()
        .route(
            "/",
            post(|state, headers, payload| async move {
                println!("Carambolas");
                admin_initiate_auth(state, headers, payload).await
            }),
        )
        .route("/cognito/signup", post(handlers::signup)) // Add the new signup route
        .with_state(store)
}
