use crate::endpoints::warp_connector_tunnel::data_structures::WarpConnectorHaConfiguration;
use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Get the HA configuration for a Warp Connector Tunnel.
/// <https://developers.cloudflare.com/api/resources/zero_trust/subresources/tunnels/subresources/warp_connector/subresources/configurations/methods/get>
#[derive(Debug)]
pub struct GetHaConfiguration<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
}

impl EndpointSpec for GetHaConfiguration<'_> {
    type JsonResponse = WarpConnectorHaConfiguration;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/warp_connector/{}/configurations",
            self.account_identifier, self.tunnel_id
        )
    }
}
