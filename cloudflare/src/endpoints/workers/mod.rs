use crate::framework::response::ApiResult;

use serde::Deserialize;

mod create_route;
mod create_secret;
mod delete_route;
mod delete_secret;
mod list_routes;
mod list_secrets;

pub use create_route::{CreateRoute, CreateRouteParams};
pub use create_secret::{CreateSecret, CreateSecretParams};
pub use delete_route::DeleteRoute;
pub use delete_secret::DeleteSecret;
pub use list_routes::ListRoutes;
pub use list_secrets::ListSecrets;

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
