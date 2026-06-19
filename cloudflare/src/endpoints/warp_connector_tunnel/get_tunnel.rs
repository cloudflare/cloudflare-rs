use crate::endpoints::warp_connector_tunnel::data_structures::WarpConnectorTunnel;
use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Fetch a single Warp Connector Tunnel.
/// <https://developers.cloudflare.com/api/resources/zero_trust/subresources/tunnels/subresources/warp_connector/methods/get>
#[derive(Debug)]
pub struct GetTunnel<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
}

impl EndpointSpec for GetTunnel<'_> {
    type JsonResponse = WarpConnectorTunnel;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/warp_connector/{}",
            self.account_identifier, self.tunnel_id
        )
    }
}
