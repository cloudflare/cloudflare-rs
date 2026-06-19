use serde::{Deserialize, Serialize};

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::{ApiResult, ApiSuccess};

/// Fetch the token used to authenticate a Warp Connector Tunnel.
/// <https://developers.cloudflare.com/api/resources/zero_trust/subresources/tunnels/subresources/warp_connector/methods/token_get>
#[derive(Debug)]
pub struct GetToken<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
}

impl EndpointSpec for GetToken<'_> {
    type JsonResponse = WarpConnectorToken;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/warp_connector/{}/token",
            self.account_identifier, self.tunnel_id
        )
    }
}

/// Tunnel token returned by the API. The raw JSON value is a plain string
/// (e.g. `"eyJhIjoi…"`) but it is wrapped here so the crate's `ApiResult`
/// trait can be implemented without violating Rust's orphan rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WarpConnectorToken(pub String);

impl ApiResult for WarpConnectorToken {}
