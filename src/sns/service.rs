use crate::memory::store::SharedStore;
use axum::{response::{IntoResponse, Response}, Json};
use hyper::StatusCode;
use serde_json::from_slice;

use super::models::*;

#[derive(Default)]
pub struct SnsService {
    store: SharedStore,
}

impl SnsService {
    pub fn new(store: SharedStore) -> Self {
        Self { store }
    }

    pub async fn create_topic(&self, name: String) -> String {
        let mut data = self.store.write().await;
        let topic_arn = format!("arn:aws:sns:local:000000000000:{}", name);
        data.sns.topics.insert(topic_arn.clone(), Topic {
            topic_arn: topic_arn.clone(),
            name,
        });
        topic_arn
    }

    pub async fn publish(&self, topic_arn: String) -> bool {
        let data = self.store.read().await;
        data.sns.topics.contains_key(&topic_arn)
    }

    pub async fn list_topics(&self) -> Vec<Topic> {
        let data = self.store.read().await;
        data.sns.topics.values().cloned().collect()
    }

    pub async fn delete_topic(&self, topic_arn: String) -> bool {
        let mut data = self.store.write().await;
        data.sns.topics.remove(&topic_arn).is_some()
    }

    pub async fn dispatch_sns(&self, x_amz_target: &str, body_bytes: impl AsRef<[u8]>) -> Response {
        match x_amz_target {
            "SNS.CreateTopic" => {
                let payload: CreateTopicRequest = match from_slice(body_bytes.as_ref()) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Failed to parse CreateTopicRequest: {e}");
                        return (StatusCode::BAD_REQUEST, "Invalid JSON for CreateTopicRequest").into_response();
                    }
                };

                let topic_arn = self.create_topic(payload.name).await;
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

                if self.publish(payload.topic_arn.clone()).await {
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
                let topics = self.list_topics().await;
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

                self.delete_topic(payload.topic_arn).await;
                Json(DeleteTopicResponse { success: true }).into_response()
            }
            _ => {
                let msg = format!("Unknown SNS X-Amz-Target: {x_amz_target}");
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
        }
    }
}
