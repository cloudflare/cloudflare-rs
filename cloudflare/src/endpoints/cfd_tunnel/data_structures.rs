use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use uuid::Uuid;

use crate::framework::response::ApiResult;

/// A Cfd Tunnel
/// This is an Cfd Tunnel that has been created. It can be used for routing and subsequent running.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Tunnel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub connections: Vec<ActiveConnection>,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct TunnelWithConnections {
    pub id: Uuid,
    pub account_tag: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub connections: Vec<ActiveConnection>,
    pub conns_active_at: Option<DateTime<Utc>>,
    pub conns_inactive_at: Option<DateTime<Utc>>,
    // tun_type can be inferred from metadata
    #[serde(flatten)]
    pub metadata: serde_json::Value,
    pub status: TunnelStatusType,
    // This field is only present for tunnels that make sense to report (e.g: Cfd_Tunnel), which
    // are the ones that can be managed via UI or dash in terms of their YAML file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_config: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TunnelStatusType {
    Inactive, // Tunnel has been created but a connection has yet to be registered
    Down,     // Tunnel is down and all connections are unregistered
    Degraded, // Tunnel health is degraded but still serving connections
    Healthy,  // Tunnel is healthy
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum ConfigurationSrc {
    #[serde(rename = "local")]
    #[default]
    Local,
    #[serde(rename = "cloudflare")]
    Cloudflare,
}
/// An active connection for a Cfd Tunnel
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
pub struct ActiveConnection {
    pub colo_name: String,
    /// Deprecated, use `id` instead.
    pub uuid: Uuid,
    pub id: Uuid,
    pub is_pending_reconnect: bool,
    pub origin_ip: IpAddr,
    pub opened_at: DateTime<Utc>,
    pub client_id: Uuid,
    pub client_version: String,
}

impl ApiResult for Tunnel {}
impl ApiResult for Vec<Tunnel> {}

/// The result of a route request for a Cfd Tunnel
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum RouteResult {
    Dns(DnsRouteResult),
    Lb(LoadBalancerRouteResult),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DnsRouteResult {
    pub cname: Change,
    pub name: String,
    pub dns_tag: String,
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
