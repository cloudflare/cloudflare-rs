use crate::framework::endpoint::{Endpoint, Method};

use super::RouteResult;
use uuid::Uuid;

/// Route for a Named Argo Tunnel
/// This creates a new route for the identified Tunnel. More than 1 route may co-exist for the same
/// Tunnel.
/// Note that this modifies only metadata on Cloudflare side to route traffic to the Tunnel, but
/// it is still up to the user to run the Tunnel to receive that traffic.
#[derive(Debug)]
pub struct RouteTunnel<'a> {
    pub zone_tag: &'a str,
    pub tunnel_id: Uuid,
    pub params: Params<'a>,
}

impl<'a> Endpoint<RouteResult, (), Params<'a>> for RouteTunnel<'a> {
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!("zones/{}/tunnels/{}/routes", self.zone_tag, self.tunnel_id)
    }
    fn body(&self) -> Option<Params<'a>> {
        Some(self.params.clone())
    }
}

/// Params for routing a Named Argo Tunnel
#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Params<'a> {
    Dns { user_hostname: &'a str },
    Lb { lb_name: &'a str, lb_pool: &'a str },
}
