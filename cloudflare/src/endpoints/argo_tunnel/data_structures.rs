use chrono::{offset::Utc, DateTime};
use uuid::Uuid;

use crate::framework::response::ApiResult;

/// A Named Argo Tunnel
/// This is an Argo Tunnel that has been created. It can be used for routing and subsequent running.
/// https://api.cloudflare.com/#argo-tunnel-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Tunnel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub connections: Vec<ActiveConnection>,
}

/// An active connection for a Named Argo Tunnel.
/// https://api.cloudflare.com/#argo-tunnel-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ActiveConnection {
    pub colo_name: String,
    pub id: Uuid,
    pub is_pending_reconnect: bool,
}

impl ApiResult for Tunnel {}

/// The result of a route request for a Named Argo Tunnel
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum RouteResult {
    Dns(DnsRouteResult),
    Lb(LoadBalancerRouteResult),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DnsRouteResult {
    pub cname: Change,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct LoadBalancerRouteResult {
    pub load_balancer: Change,
    pub pool: Change,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Change {
    Unchanged,
    New,
    Updated,
}

impl ApiResult for RouteResult {}
