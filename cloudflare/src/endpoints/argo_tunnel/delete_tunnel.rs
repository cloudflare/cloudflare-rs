use crate::framework::endpoint::{EndpointSpec, Method};

use super::Tunnel;

/// Delete a tunnel
/// <https://api.cloudflare.com/#argo-tunnel-delete-argo-tunnel>
#[derive(Debug)]
pub struct DeleteTunnel<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
    // should delete tunnel connections if any exists
    pub cascade: bool,
}

impl<'a> EndpointSpec<Tunnel> for DeleteTunnel<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/tunnels/{}?cascade={}",
            self.account_identifier, self.tunnel_id, self.cascade
        )
    }
}
