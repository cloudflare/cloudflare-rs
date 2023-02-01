use super::WorkersRoute;

use crate::framework::endpoint::{EndpointSpec, Method};

/// List Routes
/// Lists all route mappings for a given zone
/// <https://api.cloudflare.com/#worker-routes-list-routes>
#[derive(Debug)]
pub struct ListRoutes<'a> {
    pub zone_identifier: &'a str,
}

impl<'a> EndpointSpec<Vec<WorkersRoute>> for ListRoutes<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("zones/{}/workers/routes", self.zone_identifier)
    }
}
