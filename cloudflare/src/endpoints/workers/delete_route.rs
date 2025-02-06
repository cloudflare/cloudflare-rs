use super::WorkersRouteIdOnly;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Delete a Route
/// Deletes a route by route id
/// <https://api.cloudflare.com/#worker-routes-delete-route>
#[derive(Debug)]
pub struct DeleteRoute<'a> {
    pub zone_identifier: &'a str,
    pub identifier: &'a str,
}

impl<'a> EndpointSpec for DeleteRoute<'a> {
    type JsonResponse = WorkersRouteIdOnly;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/workers/routes/{}",
            self.zone_identifier, self.identifier
        )
    }
}
