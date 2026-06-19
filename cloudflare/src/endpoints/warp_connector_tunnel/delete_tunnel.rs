use crate::endpoints::warp_connector_tunnel::data_structures::WarpConnectorTunnel;
use crate::framework::endpoint::{serialize_query, EndpointSpec, Method};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Delete a WARP Connector tunnel
/// <https://developers.cloudflare.com/api/resources/zero_trust/subresources/tunnels/subresources/warp_connector/methods/delete>
#[derive(Debug)]
pub struct DeleteTunnel<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
    pub params: Params,
}

impl EndpointSpec for DeleteTunnel<'_> {
    type JsonResponse = WarpConnectorTunnel;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/warp_connector/{}",
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
