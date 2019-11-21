use super::WorkersRoute;

use crate::framework::endpoint::{Endpoint, Method};

/// Create a Route
/// Creates a route mapping the given pattern to the given script
/// https://api.cloudflare.com/#worker-routes-create-route
pub struct ListRoutes<'a> {
    pub zone_identifier: &'a str,
}

impl<'a> Endpoint<Vec<WorkersRoute>> for ListRoutes<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("zones/{}/workers/routes", self.zone_identifier)
    }
}
