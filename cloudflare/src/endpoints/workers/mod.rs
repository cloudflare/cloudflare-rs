use crate::framework::response::ApiResult;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod create_route;
mod create_secret;
mod create_tail;
mod delete_do;
mod delete_route;
mod delete_script;
mod delete_secret;
mod delete_tail;
mod list_bindings;
mod list_routes;
mod list_secrets;
mod list_tails;
mod send_tail_heartbeat;

pub use create_route::{CreateRoute, CreateRouteParams};
pub use create_secret::{CreateSecret, CreateSecretParams};
pub use create_tail::{CreateTail, CreateTailParams};
pub use delete_do::DeleteDurableObject;
pub use delete_route::DeleteRoute;
pub use delete_script::DeleteScript;
pub use delete_secret::DeleteSecret;
pub use delete_tail::DeleteTail;
pub use list_bindings::ListBindings;
pub use list_routes::ListRoutes;
pub use list_secrets::ListSecrets;
pub use list_tails::ListTails;
pub use send_tail_heartbeat::SendTailHeartbeat;

/// Workers KV Route
/// Routes are basic patterns used to enable or disable workers that match requests.
/// <https://api.cloudflare.com/#worker-routes-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersRoute {
    /// Namespace identifier tag.
    pub id: String,
    /// The basic pattern that should map to the script
    pub pattern: String,
    /// Name of the script to apply when the route is matched.
    /// The route is skipped when this is blank/missing.
    pub script: Option<String>,
}

impl ApiResult for WorkersRoute {}
impl ApiResult for Vec<WorkersRoute> {}

/// A variant of WorkersRoute returned by the CreateRoute endpoint
/// We could make `pattern` and `script` into `Option<String>` types
/// but it feels wrong.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersRouteIdOnly {
    /// Namespace identifier tag.
    pub id: String,
}

impl ApiResult for WorkersRouteIdOnly {}

/// Secrets attach to a single script to be readable in only the script
/// <https://api.cloudflare.com/#worker-secrets-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersSecret {
    pub name: String,
    #[serde(rename = "type")]
    pub secret_type: String,
}

impl ApiResult for WorkersSecret {}
impl ApiResult for Vec<WorkersSecret> {} // to parse arrays too

/// A Tail is attached to a single Worker and is impermanent
/// <https://api.cloudflare.com/#worker-tail-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersTail {
    pub id: String,
    pub url: Option<String>,
    pub expires_at: DateTime<Utc>,
}

impl ApiResult for WorkersTail {}
impl ApiResult for Vec<WorkersTail> {}

// Binding for a Workers Script
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkersBinding {
    Ai {
        name: String,
    },
    AnalyticsEngine {
        name: String,
        dataset: String,
    },
    Assets {
        name: String,
    },
    BrowserRendering {
        name: String,
    },
    D1 {
        name: String,
        id: String,
    },
    DurableObjectNamespace {
        name: String,
        class_name: String,
    },
    Hyperdrive {
        name: String,
        id: String,
    },
    KvNamespace {
        name: String,
        namespace_id: String,
    },
    MtlsCertificate {
        name: String,
        certificate_id: String,
    },
    PlainText {
        name: String,
        text: String,
    },
    Queue {
        name: String,
        queue_name: String,
    },
    R2Bucket {
        name: String,
        bucket_name: String,
    },
    SecretText {
        name: String,
        // When fetching bindings, the text field of a Secret is not returned
        text: Option<String>,
    },
    Service {
        name: String,
        service: String,
        environment: String,
    },
    TailConsumer {
        service: String,
    },
    Vectorize {
        name: String,
        index_name: String,
    },
    VersionMetadata {
        name: String,
    },
}

impl ApiResult for WorkersBinding {}
impl ApiResult for Vec<WorkersBinding> {}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::WorkersBinding;

    #[test]
    fn test_deserializing_worker_bindings() {
        // https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings
        let payload = serde_json::json!(
            [
              {
                "type": "ai",
                "name": "<VARIABLE_NAME>"
              },
              {
                "type": "analytics_engine",
                "name": "<VARIABLE_NAME>",
                "dataset": "<DATASET>"
              },
              {
                "type": "assets",
                "name": "<VARIABLE_NAME>"
              },
              {
                "type": "browser_rendering",
                "name": "<VARIABLE_NAME>"
              },
              {
                "type": "d1",
                "name": "<VARIABLE_NAME>",
                "id": "<D1_ID>"
              },
              {
                "type": "durable_object_namespace",
                "name": "<VARIABLE_NAME>",
                "class_name": "<DO_CLASS_NAME>"
              },
              {
                "type": "hyperdrive",
                "name": "<VARIABLE_NAME>",
                "id": "<HYPERDRIVE_ID>"
              },
              {
                "type": "kv_namespace",
                "name": "<VARIABLE_NAME>",
                "namespace_id": "<KV_ID>"
              },
              {
                "type": "mtls_certificate",
                "name": "<VARIABLE_NAME>",
                "certificate_id": "<MTLS_CERTIFICATE_ID>"
              },
              {
                "type": "plain_text",
                "name": "<VARIABLE_NAME>",
                "text": "<VARIABLE_VALUE>"
              },
              {
                "type": "queue",
                "name": "<VARIABLE_NAME>",
                "queue_name": "<QUEUE_NAME>"
              },
              {
                "type": "r2_bucket",
                "name": "<VARIABLE_NAME>",
                "bucket_name": "<R2_BUCKET_NAME>"
              },
              {
                "type": "secret_text",
                "name": "<VARIABLE_NAME>",
                "text": "<SECRET_VALUE>"
              },
              {
                "type": "service",
                "name": "<VARIABLE_NAME>",
                "service": "<SERVICE_NAME>",
                "environment": "production"
              },
              {
                "type": "tail_consumer",
                "service": "<WORKER_NAME>"
              },
              {
                "type": "vectorize",
                "name": "<VARIABLE_NAME>",
                "index_name": "<INDEX_NAME>"
              },
              {
                "type": "version_metadata",
                "name": "<VARIABLE_NAME>"
              }
            ]
        );

        let result: Result<Vec<WorkersBinding>, serde_json::Error> =
            serde_json::from_value(payload);
        assert!(result.is_ok());

        let mut bindings = VecDeque::from(result.unwrap());
        assert_eq!(17, bindings.len());

        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::Ai {
                name: "<VARIABLE_NAME>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::AnalyticsEngine {
                name: "<VARIABLE_NAME>".to_string(),
                dataset: "<DATASET>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::Assets {
                name: "<VARIABLE_NAME>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::BrowserRendering {
                name: "<VARIABLE_NAME>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::D1 {
                name: "<VARIABLE_NAME>".to_string(),
                id: "<D1_ID>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::DurableObjectNamespace {
                name: "<VARIABLE_NAME>".to_string(),
                class_name: "<DO_CLASS_NAME>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::Hyperdrive {
                name: "<VARIABLE_NAME>".to_string(),
                id: "<HYPERDRIVE_ID>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::KvNamespace {
                name: "<VARIABLE_NAME>".to_string(),
                namespace_id: "<KV_ID>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::MtlsCertificate {
                name: "<VARIABLE_NAME>".to_string(),
                certificate_id: "<MTLS_CERTIFICATE_ID>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::PlainText {
                name: "<VARIABLE_NAME>".to_string(),
                text: "<VARIABLE_VALUE>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::Queue {
                name: "<VARIABLE_NAME>".to_string(),
                queue_name: "<QUEUE_NAME>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::R2Bucket {
                name: "<VARIABLE_NAME>".to_string(),
                bucket_name: "<R2_BUCKET_NAME>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::SecretText {
                name: "<VARIABLE_NAME>".to_string(),
                text: Some("<SECRET_VALUE>".to_string())
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::Service {
                name: "<VARIABLE_NAME>".to_string(),
                service: "<SERVICE_NAME>".to_string(),
                environment: "production".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::TailConsumer {
                service: "<WORKER_NAME>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::Vectorize {
                name: "<VARIABLE_NAME>".to_string(),
                index_name: "<INDEX_NAME>".to_string()
            }
        );
        assert_eq!(
            bindings.pop_front().unwrap(),
            WorkersBinding::VersionMetadata {
                name: "<VARIABLE_NAME>".to_string()
            }
        );

        assert!(bindings.is_empty());
    }
}
