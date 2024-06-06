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
pub struct CreateTunnel<'a> {
    pub account_identifier: &'a str,
    pub params: Params<'a>,
}

impl<'a> EndpointSpec<Tunnel> for CreateTunnel<'a> {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("accounts/{}/cfd_tunnel", self.account_identifier)
    }
    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}

/// Params for creating a Named Argo Tunnel
#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Params<'a> {
    /// The name for the Tunnel to be created. It must be unique within the account.
    pub name: &'a str,
    /// The byte array (with 32 or more bytes) representing a secret for the tunnel. This is
    /// encoded into JSON as a base64 String. This secret is necessary to run the tunnel.
    #[serde_as(as = "Base64<Standard, Padded>")]
    pub tunnel_secret: &'a Vec<u8>,

    pub config_src: &'a ConfigurationSrc,

    /// Arbitrary metadata for the tunnel.
    pub metadata: Option<serde_json::Value>,
}
