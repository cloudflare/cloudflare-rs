use super::WorkersRouteIdOnly;

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};

use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Create a Route
/// Creates a route mapping the given pattern to the given script
/// <https://api.cloudflare.com/#worker-routes-create-route>
#[derive(Debug)]
pub struct CreateRoute<'a> {
    pub zone_identifier: &'a str,
    pub params: CreateRouteParams,
}

impl<'a> EndpointSpec for CreateRoute<'a> {
    type JsonResponse = WorkersRouteIdOnly;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/workers/routes", self.zone_identifier)
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
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
