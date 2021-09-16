use super::WorkersRouteIdOnly;

use crate::framework::endpoint::{Endpoint, Method};

/// Create a Route
/// Creates a route mapping the given pattern to the given script
/// https://api.cloudflare.com/#worker-routes-create-route
#[derive(Debug)]
pub struct CreateRoute<'a> {
    pub zone_identifier: &'a str,
    pub params: CreateRouteParams,
}

impl<'a> Endpoint<WorkersRouteIdOnly, (), CreateRouteParams> for CreateRoute<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("zones/{}/workers/routes", self.zone_identifier)
    }
    fn body(&self) -> Option<CreateRouteParams> {
        Some(self.params.clone())
    }
}

/// pattern: the zone name along with glob-style wildcards
///         e.g. "example.net/*"
/// script: Name of the script to apply when the route is matched.
///         The route is skipped when this is blank/missing.
#[derive(Serialize, Clone, Debug)]
pub struct CreateRouteParams {
    pub pattern: String,
    pub script: Option<String>,
}
