use crate::endpoints::warp_connector_tunnel::data_structures::WarpConnector;
use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Fetch connector + connection details for a single Warp Connector client.
/// <https://developers.cloudflare.com/api/resources/zero_trust/subresources/tunnels/subresources/warp_connector/subresources/connectors/methods/get>
#[derive(Debug)]
pub struct GetConnector<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
    pub connector_id: &'a str,
}

impl EndpointSpec for GetConnector<'_> {
    type JsonResponse = WarpConnector;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/warp_connector/{}/connectors/{}",
            self.account_identifier, self.tunnel_id, self.connector_id
        )
    }
}
