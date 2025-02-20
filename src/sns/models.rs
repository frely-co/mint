use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Topic {
    pub topic_arn: String,
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct PublishResult {
    #[serde(rename = "MessageId")]
    pub message_id: Option<String>,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct PublishResponse {
    #[serde(rename = "PublishResult")]
    pub publish_result: PublishResult,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct CreateTopicResponse {
    #[serde(rename = "TopicArn")]
    pub topic_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct ListTopicsResponse {
    #[serde(rename = "Topics")]
    pub topics: Vec<Topic>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct DeleteTopicResponse {
    #[serde(rename = "Success")]
    pub success: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct CreateTopicRequest {
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct PublishRequest {
    #[serde(rename = "TopicArn")]
    pub topic_arn: String,
    #[serde(rename = "Message")]
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct DeleteTopicRequest {
    #[serde(rename = "TopicArn")]
    pub topic_arn: String,
}
