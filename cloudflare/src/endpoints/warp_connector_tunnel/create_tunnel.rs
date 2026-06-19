use serde::Serialize;
use serde_with::serde_as;

use crate::framework::response::ApiSuccess;
use crate::{
    endpoints::warp_connector_tunnel::data_structures::WarpConnectorTunnel,
    framework::endpoint::{EndpointSpec, Method, RequestBody},
};

/// Create a Warp Connector tunnel
/// <https://developers.cloudflare.com/api/resources/zero_trust/subresources/tunnels/subresources/warp_connector/methods/create>
#[derive(Debug)]
pub struct CreateTunnel<'a> {
    pub account_identifier: &'a str,
    pub params: Params<'a>,
}

impl EndpointSpec for CreateTunnel<'_> {
    type JsonResponse = WarpConnectorTunnel;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("accounts/{}/warp_connector", self.account_identifier)
    }
    #[inline]
    fn body(&'_ self) -> Option<RequestBody<'_>> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Params for creating a Warp Connector Tunnel
#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Params<'a> {
    /// The name for the Tunnel to be created. It must be unique within the account.
    pub name: &'a str,
    pub ha: bool,
}
