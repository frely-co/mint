pub mod models;
pub mod repository;

use crate::memory::store::SharedStore;
use axum::{response::{IntoResponse, Response}, Json};
use hyper::StatusCode;
use serde_json::from_slice;
use self::models::*;

pub async fn dispatch_sns(
    x_amz_target: &str,
    store: SharedStore,
    body_bytes: impl AsRef<[u8]>,
) -> Response {
    match x_amz_target {
        "SNS.CreateTopic" => {
            let payload: CreateTopicRequest = match from_slice(body_bytes.as_ref()) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to parse CreateTopicRequest: {e}");
                    return (StatusCode::BAD_REQUEST, "Invalid JSON for CreateTopicRequest").into_response();
                }
            };

            let mut data = store.write().await;
            let topic_arn = format!("arn:aws:sns:local:000000000000:{}", payload.name);
            data.sns.topics.insert(topic_arn.clone(), Topic {
                topic_arn: topic_arn.clone(),
                name: payload.name,
            });

            Json(CreateTopicResponse { topic_arn }).into_response()
        }
        "SNS.Publish" => {
            let payload: PublishRequest = match from_slice(body_bytes.as_ref()) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to parse PublishRequest: {e}");
                    return (StatusCode::BAD_REQUEST, "Invalid JSON for PublishRequest").into_response();
                }
            };

            let data = store.read().await;
            if data.sns.topics.contains_key(&payload.topic_arn) {
                Json(PublishResponse {
                    publish_result: PublishResult {
                        message_id: Some(uuid::Uuid::new_v4().to_string()),
                        sequence_number: None,
                    },
                }).into_response()
            } else {
                (StatusCode::NOT_FOUND, "Topic not found").into_response()
            }
        }
        "SNS.ListTopics" => {
            let data = store.read().await;
            let topics = data.sns.topics.values().cloned().collect();
            Json(ListTopicsResponse { topics }).into_response()
        }
        "SNS.DeleteTopic" => {
            let payload: DeleteTopicRequest = match from_slice(body_bytes.as_ref()) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to parse DeleteTopicRequest: {e}");
                    return (StatusCode::BAD_REQUEST, "Invalid JSON for DeleteTopicRequest").into_response();
                }
            };

            let mut data = store.write().await;
            data.sns.topics.remove(&payload.topic_arn);
            Json(DeleteTopicResponse { success: true }).into_response()
        }
        _ => {
            let msg = format!("Unknown SNS X-Amz-Target: {x_amz_target}");
            (StatusCode::BAD_REQUEST, msg).into_response()
        }
    }
}
