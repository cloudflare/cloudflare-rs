use crate::framework::response::ApiResult;

use chrono::offset::Utc;
use chrono::DateTime;
use serde::Deserialize;

mod create_route;
mod create_secret;
mod create_tail;
mod create_tail_heartbeat;
mod delete_route;
mod delete_secret;
mod delete_tail;
mod list_routes;
mod list_secrets;
mod list_tails;

pub use create_route::{CreateRoute, CreateRouteParams};
pub use create_secret::{CreateSecret, CreateSecretParams};
pub use create_tail::{CreateTail, CreateTailParams};
pub use create_tail_heartbeat::{CreateTailHeartbeat};
pub use delete_route::DeleteRoute;
pub use delete_secret::DeleteSecret;
pub use delete_tail::DeleteTail;
pub use list_routes::ListRoutes;
pub use list_secrets::ListSecrets;
pub use list_tails::ListTails;

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
    /// TODO: these fields depend on the API and may be wrong since unable to test
    pub name: String,
    #[serde(rename = "type")]
    pub secret_type: String,
    pub modified_on: DateTime<Utc>,
    pub created_on: DateTime<Utc>,
}

impl ApiResult for WorkersSecret {}
impl ApiResult for Vec<WorkersSecret> {} // to parse arrays too

/// A Tail is attached to a single Worker and is impermanent
/// https://api.cloudflare.com/#worker-tail-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersTail {
    pub id: String,
    pub url: String,
    pub expires_at: DateTime<Utc>,
}

impl ApiResult for WorkersTail {}
impl ApiResult for Vec<WorkersTail> {}
