use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::json_utils::serialize_base64_str;

use super::Tunnel;

/// Create a Named Argo Tunnel
/// This creates the Tunnel, which can then be routed and ran. Creating the Tunnel per se is only
/// a metadata operation (i.e. no Tunnel is running at this point).
/// https://api.cloudflare.com/#argo-tunnel-create-argo-tunnel
#[derive(Debug)]
pub struct CreateTunnel<'a> {
    pub account_identifier: &'a str,
    pub params: Params<'a>,
}

impl<'a> Endpoint<Tunnel, (), Params<'a>> for CreateTunnel<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("accounts/{}/tunnels", self.account_identifier)
    }
    fn body(&self) -> Option<Params<'a>> {
        Some(self.params.clone())
    }
}

/// Params for creating a Named Argo Tunnel
#[derive(Serialize, Clone, Debug)]
pub struct Params<'a> {
    /// The name for the Tunnel to be created. It must be unique within the account.
    pub name: &'a str,
    /// The byte array (with 32 or more bytes) representing a secret for the tunnel. This is
    /// encoded into JSON as a base64 String. This secret is necessary to run the tunnel.
    #[serde(serialize_with = "serialize_base64_str")]
    pub tunnel_secret: &'a Vec<u8>,
}
