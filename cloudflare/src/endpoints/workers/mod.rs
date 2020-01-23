use crate::framework::response::ApiResult;

use serde::Deserialize;

mod create_route;
pub mod create_secret;
mod delete_route;
pub mod delete_secret;
mod list_routes;

pub use create_route::{CreateRoute, CreateRouteParams};
pub use delete_route::DeleteRoute;
pub use list_routes::ListRoutes;

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
  /// TODO these fields depend on the API and may be wrong
  pub name: String,
  pub modified_on: String,
  pub script_id: String,
}

impl ApiResult for WorkersSecret {}
impl ApiResult for Vec<WorkersSecret> {} // to parse arrays too
