use serde::{Deserialize, Serialize};

/// <p>Response for Publish action.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[derive(Serialize)]
pub struct PublishResult {
    /// <p>Unique identifier assigned to the published message.</p> <p>Length Constraint: Maximum 100 characters</p>
    #[serde(rename = "MessageId")]
    pub message_id: Option<String>,
    /// <p>This response element applies only to FIFO (first-in-first-out) topics. </p> <p>The sequence number is a large, non-consecutive number that Amazon SNS assigns to each message. The length of <code>SequenceNumber</code> is 128 bits. <code>SequenceNumber</code> continues to increase for each <code>MessageGroupId</code>.</p>
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: Option<String>,
}

/// Represents the response structure for a publish operation.
#[derive(Clone, Debug, Default, PartialEq)]
#[derive(Serialize)]
pub struct PublishResponse {
    /// Contains the result of the publish operation.
    #[serde(rename = "PublishResult")]
    pub publish_result: PublishResult,
}
