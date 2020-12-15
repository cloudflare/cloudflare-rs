use super::WorkersRouteIdOnly;

use crate::framework::endpoint::{Endpoint, Method};

/// Delete a Route
/// Deletes a route by route id
/// https://api.cloudflare.com/#worker-routes-delete-route
#[derive(Debug)]
pub struct DeleteRoute<'a> {
    pub zone_identifier: &'a str,
    pub identifier: &'a str,
}

impl<'a> Endpoint<WorkersRouteIdOnly> for DeleteRoute<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/workers/routes/{}",
            self.zone_identifier, self.identifier
        )
    }
}
