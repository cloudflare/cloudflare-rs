use crate::endpoints::cfd_tunnel::{ConfigurationSrc, Tunnel};
use serde::Serialize;
use serde_with::{
    base64::{Base64, Standard},
    formats::Padded,
    serde_as,
};

use crate::framework::endpoint::{EndpointSpec, Method};

/// Create a Cfd Tunnel
/// This creates the Tunnel, which can then be routed and ran. Creating the Tunnel per se is only
/// a metadata operation (i.e. no Tunnel is running at this point).
/// <https://developers.cloudflare.com/api/operations/cloudflare-tunnel-create-a-cloudflare-tunnel>
#[derive(Debug)]
pub struct GetTunnel<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
}

impl<'a> EndpointSpec<Tunnel> for GetTunnel<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/cfd_tunnel/{}",
            self.account_identifier, self.tunnel_id
        )
    }
    #[inline]
    fn body(&self) -> Option<String> {
        None
    }
}
