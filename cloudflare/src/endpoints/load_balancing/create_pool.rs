use crate::endpoints::load_balancing::{Origin, Pool};
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};

use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Create Pool
/// <https://api.cloudflare.com/#account-load-balancer-pools-create-pool>
#[derive(Debug)]
pub struct CreatePool<'a> {
    /// The Cloudflare account to create this Pool under.
    pub account_identifier: &'a str,
    /// Optional parameters for the API call
    pub params: Params<'a>,
}

/// Mandatory parameters for creating a Load Balancer Pool.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Params<'a> {
    /// A short name (tag) for the pool.
    /// Only alphanumeric characters, hyphens and underscores are allowed.
    /// E.g. "primary-dc-1"
    pub name: &'a str,
    /// The list of origins within this pool.
    /// Traffic directed at this pool is balanced across all currently healthy origins, provided
    /// the pool itself is healthy.
    pub origins: &'a [Origin],
    #[serde(flatten)]
    pub optional_params: Option<OptionalParams<'a>>,
}

/// Optional parameters for creating a Load Balancer Pool.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct OptionalParams<'a> {
    /// A human-readable description of the pool.
    /// e.g. "Primary data center - Provider XYZ"
    pub description: Option<&'a str>,
    /// Whether to enable (the default) this pool. Disabled pools will not receive traffic and are
    /// excluded from health checks. Disabling a pool will cause any load balancers using it to
    /// failover to the next pool (if any).
    pub enabled: Option<bool>,
    /// The minimum number of origins that must be healthy for this pool to serve traffic. If the
    /// number of healthy origins falls below this number, the pool will be marked unhealthy and we
    /// will failover to the next available pool.
    pub minimum_origins: Option<u8>,
    /// The ID of the Monitor to use for health checking origins within this pool.
    pub monitor: Option<&'a str>,
    /// The email address to send health status notifications to. This can be an individual mailbox
    /// or a mailing list.
    pub notification_email: Option<&'a str>,
}

impl EndpointSpec for CreatePool<'_> {
    type JsonResponse = Pool;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("accounts/{}/load_balancers/pools", self.account_identifier)
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}
