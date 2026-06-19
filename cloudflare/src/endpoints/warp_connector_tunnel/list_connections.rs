use crate::endpoints::warp_connector_tunnel::data_structures::WarpConnector;
use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// List active connections for a Warp Connector Tunnel.
/// <https://developers.cloudflare.com/api/resources/zero_trust/subresources/tunnels/subresources/warp_connector/subresources/connections/methods/list>
#[derive(Debug)]
pub struct ListConnections<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
}

impl EndpointSpec for ListConnections<'_> {
    type JsonResponse = Vec<WarpConnector>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/warp_connector/{}/connections",
            self.account_identifier, self.tunnel_id
        )
    }
}
