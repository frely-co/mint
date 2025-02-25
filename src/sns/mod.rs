pub mod models;
pub mod repository;
pub mod service;

use crate::memory::store::SharedStore;
use axum::{response::{IntoResponse, Response}, Json};
use hyper::StatusCode;
use serde_json::from_slice;
use self::models::*;
use self::service::SnsService;

pub async fn dispatch_sns(
    x_amz_target: &str,
    store: SharedStore,
    body_bytes: impl AsRef<[u8]>,
) -> Response {
    let service = SnsService::new(store);
    service.dispatch_sns(x_amz_target, body_bytes).await
}
