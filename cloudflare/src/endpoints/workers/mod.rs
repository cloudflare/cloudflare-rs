use crate::framework::response::ApiResult;

use chrono::{DateTime, Utc};
use serde::Deserialize;

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
/// https://api.cloudflare.com/#worker-routes-properties
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
/// https://api.cloudflare.com/#worker-secrets-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersSecret {
    pub name: String,
    #[serde(rename = "type")]
    pub secret_type: String,
}

impl ApiResult for WorkersSecret {}
impl ApiResult for Vec<WorkersSecret> {} // to parse arrays too

/// A Tail is attached to a single Worker and is impermanent
/// https://api.cloudflare.com/#worker-tail-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersTail {
    pub id: String,
    pub url: Option<String>,
    pub expires_at: DateTime<Utc>,
}

impl ApiResult for WorkersTail {}
impl ApiResult for Vec<WorkersTail> {}

// Binding for a Workers Script
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersBinding {
    pub name: String,
    pub r#type: String,
    pub namespace_id: String,
    pub class_name: Option<String>,
}

impl ApiResult for WorkersBinding {}
impl ApiResult for Vec<WorkersBinding> {}
