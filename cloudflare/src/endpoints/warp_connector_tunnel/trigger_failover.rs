use serde::{Deserialize, Serialize};

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};

/// Trigger a manual failover for a Warp Connector Tunnel, promoting a
/// specific client to be the active connector. The tunnel must be configured
/// for HA and the client must already be linked to it.
/// <https://developers.cloudflare.com/api/resources/zero_trust/subresources/tunnels/subresources/warp_connector/subresources/failover/methods/update>
#[derive(Debug)]
pub struct TriggerFailover<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
    pub params: Params<'a>,
}

impl EndpointSpec for TriggerFailover<'_> {
    type JsonResponse = FailoverResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/warp_connector/{}/failover",
            self.account_identifier, self.tunnel_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody<'_>> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Params for triggering a manual failover.
#[derive(Serialize, Clone, Debug)]
pub struct Params<'a> {
    /// UUID of the Cloudflare Tunnel connector to promote.
    pub client_id: &'a str,
}

/// The failover endpoint's `result` field is documented as `unknown` and the
/// example response is the empty object `{}`. Wrap a raw JSON value so the
/// caller can still inspect anything the API returns.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FailoverResponse(pub serde_json::Value);

impl ApiResult for FailoverResponse {}
