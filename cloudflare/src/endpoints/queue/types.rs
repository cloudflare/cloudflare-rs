use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProducerType {
    Worker,
    R2Bucket,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct MqWorkerProducer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(rename = "type")]
    pub type_: ProducerType,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct MqR2Producer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "type")]
    pub type_: ProducerType,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Producer {
    Worker(MqWorkerProducer),
    R2Bucket(MqR2Producer),
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct QueueSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_delay: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_retention_period: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Queue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumers: Option<Vec<String>>, // Assuming Consumer is just a string for this example
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumers_total_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producers: Option<Vec<Producer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producers_total_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<QueueSettings>,
}
