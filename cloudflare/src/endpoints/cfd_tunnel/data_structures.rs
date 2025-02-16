use crate::framework::response::ApiResult;
use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TunnelToken(String);

impl From<String> for TunnelToken {
    fn from(s: String) -> TunnelToken {
        TunnelToken(s)
    }
}

impl From<TunnelToken> for String {
    fn from(s: TunnelToken) -> String {
        s.0
    }
}

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

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TunnelConfiguration {
    pub ingress: Option<Vec<Ingress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub origin_request: Option<OriginRequest>,
    #[serde(rename = "warp-routing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub warp_routing: Option<WarpRouting>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
pub struct WarpRouting {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Ingress {
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "warp-routing")]
    pub origin_request: Option<OriginRequest>,
    pub path: Option<String>,
    pub service: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct OriginRequestAccress {
    pub aud_tag: Vec<String>,
    pub required: bool,
    pub team_name: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct OriginRequest {
    pub access: Option<OriginRequestAccress>,
    pub ca_pool: Option<String>,
    pub connection_timeout: i32,
    pub disable_chunked_encoding: bool,
    pub http2origin: bool,
    pub http_host_header: Option<String>,
    pub keep_alive_connections: i32,
    pub keep_alive_timeout: i32,
    pub no_happy_eyeballs: bool,
    pub no_tls_verify: bool,
    pub origin_server_name: Option<String>,
    pub proxy_type: Option<String>,
    pub tcp_keep_alive: i32,
    pub tls_timeout: i32,
}

impl ApiResult for TunnelConfiguration {}
impl ApiResult for Tunnel {}
impl ApiResult for Vec<Tunnel> {}
impl ApiResult for TunnelToken {}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TunnelConfigurationResult {
    pub account_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub config: Option<TunnelConfiguration>,
    pub source: String,
    pub tunnel_id: Uuid,
    pub version: i32,
}

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

impl ApiResult for TunnelConfigurationResult {}
impl ApiResult for RouteResult {}
