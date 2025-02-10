use core::panic;

use crate::dynamodb::models::PutItemOutput;
use crate::sns::models::PublishResult;
use crate::{cognito::service as cognito_service, sns::models::PublishResponse};
use crate::memory::store::SharedStore;
use axum::{
    body::Bytes, extract::State, http::HeaderMap, response::Response, routing::post, Router,
};
use serde_json::from_slice;

pub fn create_router(store: SharedStore) -> Router {
    Router::new()
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
    let x_amz_target = headers
        .get("X-Amz-Target")
        .and_then(|val| val.to_str().ok())
        .unwrap_or("");

    let content_type = headers
        .get("Content-Type")
        .and_then(|val| val.to_str().ok())
        .unwrap_or("");

    eprintln!("Headers: {:?}", headers);
    eprintln!("X-Amz-Target: {}", x_amz_target);
    eprintln!("Content-Type: {}", content_type);
    eprintln!("Body: {:?}", body_bytes);

match content_type {
    "application/x-amz-json-1.1" => {
        if x_amz_target.starts_with("AWSCognitoIdentityProviderService") {
            cognito_service::dispatch_cognito(x_amz_target, store, body_bytes).await
        } else {
            panic!("error");
        }
    }
    "application/x-www-form-urlencoded" => {
        if let Ok(body_str) = String::from_utf8(body_bytes.to_vec()) {
            if body_str.contains("Action=Publish") {
                let message = body_str
                    .split('&')
                    .find(|s| s.starts_with("Message="))
                    .and_then(|s| s.split('=').nth(1))
                    .map(|s| urlencoding::decode(s).unwrap_or_default().into_owned())
                    .unwrap_or_default();

                eprintln!("Received SNS Publish request with message: {}", message);
                let response = PublishResponse {
                    publish_result: PublishResult {
                        message_id: Some("some_message_id".to_string()),
                        sequence_number: Some("some_sequence_number".to_string()),
                    },
                };
                return Response::builder()
                    .status(200)
                    .body(serde_xml_rs::to_string(&response).unwrap().into())
                    .unwrap();
            }
        }
        eprintln!("Received SNS request, but SNS handling is not implemented.");
        Response::builder()
            .status(200)
            .body("SNS request received, but not handled.".into())
            .unwrap()
    }
    "application/x-amz-json-1.0" => {
        if x_amz_target == "DynamoDB_20120810.PutItem" {
            let payload: crate::dynamodb::models::PutItemRequest = match from_slice(&body_bytes) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to parse PutItemRequest: {e}");
                    return (
                        StatusCode::BAD_REQUEST,
                        "Invalid JSON for PutItemRequest",
                    )
                        .into_response();
                }
            };

            let mut data = store.write().await;
            data.dynamo.put_item(&payload.table_name, payload.item);

            let put_item_output = PutItemOutput {
                attributes: None,
                consumed_capacity: None,
                item_collection_metrics: None,
            };
            Response::builder()
                .status(200)
                .body(serde_json::to_string(&put_item_output).unwrap().into())
                .unwrap()
        } else {
            panic!("Unsupported X-Amz-Target: {}", x_amz_target);
        }
    }
    _ => panic!("Unsupported Content-Type: {}", content_type),
}
}
