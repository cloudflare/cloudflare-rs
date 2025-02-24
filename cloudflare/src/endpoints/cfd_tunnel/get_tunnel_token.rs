use super::TunnelToken as TunnelTokenResult;
use crate::framework::endpoint::{EndpointSpec, Method};

/// Delete a tunnel
/// <https://developers.cloudflare.com/api/operations/cloudflare-tunnel-delete-a-cloudflare-tunnel>
#[derive(Debug)]
pub struct TunnelToken<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
}

impl<'a> EndpointSpec<TunnelTokenResult> for TunnelToken<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/cfd_tunnel/{}/token",
            self.account_identifier, self.tunnel_id
        )
    }
    #[inline]
    fn body(&self) -> Option<String> {
        None
    }
}
