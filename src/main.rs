use mint::{
    memory::store::{MemoryStore, SharedStore},
    server::create_router,
};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::RwLock;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Construct the app: a single POST endpoint that simulates the AWS Cognito endpoint.
    let store = SharedStore::new(RwLock::new(MemoryStore::default()));

    let app = create_router(store);

    // Run the server on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Mock AWS service running at http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
