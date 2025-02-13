use super::Tunnel;
use crate::framework::endpoint::{serialize_query, EndpointSpec, Method};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Delete a tunnel
/// <https://developers.cloudflare.com/api/operations/cloudflare-tunnel-delete-a-cloudflare-tunnel>
#[derive(Debug)]
pub struct DeleteTunnel<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
    pub params: Params,
}

impl<'a> EndpointSpec for DeleteTunnel<'a> {
    type JsonResponse = Tunnel;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/cfd_tunnel/{}",
            self.account_identifier, self.tunnel_id
        )
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct Params {
    // should delete tunnel connections if any exists
    pub cascade: bool,
}
