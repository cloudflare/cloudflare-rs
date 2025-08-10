use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Consumer {
    Worker(MqWorkerConsumer),
    HttpPull(MqHttpConsumer),
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct MqWorkerConsumer {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<String>, // Optional (maxLength: 32)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_queue: Option<String>,

    /// A Resource identifier.
    /// Optional (maxLength: 32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>, // Optional (maxLength: 32)

    /// Name of a Worker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

    /// Name of a Worker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WorkerSettings>,

    #[serde(rename = "type")]
    pub type_: Option<ConsumerType>, // "worker"
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerType {
    Worker,
    HttpPull,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct WorkerSettings {
    /// The maximum number of messages to include in a batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,

    /// Maximum number of concurrent consumers that may consume from this Queue. Set to null to automatically opt in to the platform's maximum (recommended).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<u32>,

    /// The maximum number of retries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,

    /// The number of milliseconds to wait for a batch to fill up before attempting to deliver it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wait_time_ms: Option<u64>,

    /// The number of seconds to delay before making the message available for another attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct MqHttpConsumer {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<String>, // Optional (maxLength: 32)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_queue: Option<String>,

    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>, // Optional (maxLength: 32)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<HttpSettings>,

    #[serde(rename = "type")]
    pub type_: Option<ConsumerType>, // "http_pull"
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct HttpSettings {
    /// The maximum number of messages to include in a batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,

    /// The maximum number of retries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,

    /// The number of seconds to delay before making the message available for another attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay: Option<u32>,

    /// The number of milliseconds that a message is exclusively leased. After the timeout, the message becomes available for another attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_timeout_ms: Option<u64>,
}
