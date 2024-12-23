use core::panic;

use crate::cognito::service as cognito_service;
use crate::memory::store::SharedStore;
use axum::{
    body::Bytes, extract::State, http::HeaderMap, response::Response, routing::post, Router,
};
// use crate::s3::service as s3_service;
// use crate::dynamodb::service as dynamodb_service;

pub fn create_router(store: SharedStore) -> Router {
    Router::new()
        // You can also use .route("/*path", post(dispatch_request)) if you
        // need to match real AWS endpoints or signatures.
        .route("/", post(dispatch_request))
        .with_state(store)
}

/// This handler is a single entry point that checks `X-Amz-Target`
/// (or other info) and dispatches to the right “service.”
async fn dispatch_request(
    State(store): State<SharedStore>,
    headers: HeaderMap,
    body_bytes: Bytes,
) -> Response {
    // For now, just look at `X-Amz-Target`
    let x_amz_target = headers
        .get("X-Amz-Target")
        .and_then(|val| val.to_str().ok())
        .unwrap_or("");

    // You can parse the prefix before the first dot if needed
    // e.g. "AWSCognitoIdentityProviderService.AdminInitiateAuth" => "AWSCognitoIdentityProviderService"
    // and then dispatch to the correct service
    if x_amz_target.starts_with("AWSCognitoIdentityProviderService") {
        cognito_service::dispatch_cognito(x_amz_target, store, body_bytes).await
    // } else if x_amz_target.starts_with("DynamoDB_") {
    //     dynamodb_service::dispatch_dynamodb(x_amz_target, store, body_bytes).await
    // } else if x_amz_target.starts_with("AmazonS3") {
    //     s3_service::dispatch_s3(x_amz_target, store, body_bytes).await
    } else {
        // Unknown or unsupported
        panic!("error");
        // let msg = format!("Unknown X-Amz-Target: {x_amz_target}");
        // return (StatusCode::NOT_FOUND, msg).into_response();
    }
}

