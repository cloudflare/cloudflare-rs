use serde::Serialize;

use crate::endpoints::warp_connector_tunnel::data_structures::{
    WarpConnectorHaConfiguration, WarpConnectorHaMode, WarpConnectorProviderConfiguration,
};
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

/// Add or update the HA configuration for a Warp Connector Tunnel.
/// <https://developers.cloudflare.com/api/resources/zero_trust/subresources/tunnels/subresources/warp_connector/subresources/configurations/methods/update>
#[derive(Debug)]
pub struct UpdateHaConfiguration<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
    pub params: Params,
}

impl EndpointSpec for UpdateHaConfiguration<'_> {
    type JsonResponse = WarpConnectorHaConfiguration;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/warp_connector/{}/configurations",
            self.account_identifier, self.tunnel_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody<'_>> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Params for updating the HA configuration.
///
/// `config` is required for `aws` and `local` modes and must be omitted (or
/// `None`) for `none` and `disabled`.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Params {
    pub ha_mode: WarpConnectorHaMode,
    pub config: Option<WarpConnectorProviderConfiguration>,
}
