use crate::{json_content, Endpoint, JsonResponse, Method};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::net::IpAddr;

/// A pool is a set of origins that requests could be routed to (e.g. each of your data centers or
/// regions have its own pool).
/// Requests will be routed to particular pools according to your steering policy, and then balanced
/// across origins in that pool, proportional to each origin's weight.
///
/// For example, you might have two pools: one for the US, and one for Oceania. Inside each pool,
/// there would be many origins, with weight roughly proportional to the number of requests they can
/// handle. Then you might use a "dynamic latency" steering policy to ensure requests get routed
/// to whatever pool can serve them fastest. So US users will probably get routed to the US pool. If
/// the US pool becomes unavailable, they'll fail over to the Oceania pool.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Pool {
    pub id: String,
    pub created_on: DateTime<Utc>,
    pub modified_on: DateTime<Utc>,
    /// A human-readable description of the pool.
    /// e.g. "Primary data center - Provider XYZ"
    pub description: String,
    pub name: String,
    /// Whether to enable (the default) this pool. Disabled pools will not receive traffic and are
    /// excluded from health checks. Disabling a pool will cause any load balancers using it to
    /// failover to the next pool (if any).
    pub enabled: bool,
    /// The minimum number of origins that must be healthy for this pool to serve traffic. If the
    /// number of healthy origins falls below this number, the pool will be marked unhealthy and we
    /// will failover to the next available pool.
    pub minimum_origins: u8,
    /// The ID of the Monitor to use for health checking origins within this pool.
    pub monitor: String,
    pub check_regions: Option<Vec<String>>,
    /// The list of origins within this pool. Traffic directed at this pool is balanced across all
    /// currently healthy origins, provided the pool itself is healthy.
    pub origins: Vec<Origin>,
    /// The email address to send health status notifications to. This can be an individual mailbox
    /// or a mailing list.
    pub notification_email: String,
}

/// An origin represents something that can serve user requests. Usually a machine, maybe an ELB.
/// Origins with similar latency functions (e.g. origins in the same data center or region) might be
/// in the same pool.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Origin {
    /// A human-identifiable name for the origin.
    /// e.g. app-server-1
    pub name: String,
    /// The IP address (IPv4 or IPv6) of the origin, or the publicly addressable hostname.
    /// Hostnames entered here should resolve directly to the origin, and not be a hostname proxied
    /// by Cloudflare.
    /// e.g. 0.0.0.0
    pub address: IpAddr,
    /// Whether to enable (the default) this origin within the Pool. Disabled origins will not
    /// receive traffic and are excluded from health checks. The origin will only be disabled for
    /// the current pool.
    pub enabled: bool,
    /// The weight of this origin relative to other origins in the Pool. Based on the configured
    /// weight the total traffic is distributed among origins within the Pool.
    pub weight: f64,
}

/// Pool Details
///
/// https://api.cloudflare.com/#account-load-balancer-pools-pool-details
#[derive(Debug, Clone, PartialEq)]
pub struct Get<'a> {
    /// The Cloudflare account of this pool.
    pub account_id: &'a str,
    /// Which pool to retrieve the details of.
    pub pool_id: &'a str,
}

impl Endpoint for Get<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<Pool>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!("accounts/{}/load_balancers/pools", self.account_id).into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// Create Pool
/// https://api.cloudflare.com/#account-load-balancer-pools-create-pool
#[derive(Debug, Clone, PartialEq)]
pub struct Create<'a> {
    /// The Cloudflare account to create this Pool under.
    pub account_id: &'a str,
    /// Parameters for the API call
    pub params: CreateParams,
}

impl Endpoint for Create<'_> {
    type Body = CreateParams;
    type Query = ();
    type Response = JsonResponse<Pool>;

    const METHOD: Method = Method::Post;

    fn path(&self) -> Cow<str> {
        format!("accounts/{}/load_balancers/pools", self.account_id).into()
    }
    fn body(&self) -> &Self::Body {
        &self.params
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// Mandatory parameters for creating a Load Balancer Pool.
#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateParams {
    /// A short name (tag) for the pool.
    /// Only alphanumeric characters, hyphens and underscores are allowed.
    /// E.g. "primary-dc-1"
    pub name: String,
    /// The list of origins within this pool.
    /// Traffic directed at this pool is balanced across all currently healthy origins, provided
    /// the pool itself is healthy.
    pub origins: Vec<Origin>,
    /// A human-readable description of the pool.
    /// e.g. "Primary data center - Provider XYZ"
    pub description: Option<String>,
    /// Whether to enable (the default) this pool. Disabled pools will not receive traffic and are
    /// excluded from health checks. Disabling a pool will cause any load balancers using it to
    /// failover to the next pool (if any).
    pub enabled: Option<bool>,
    /// The minimum number of origins that must be healthy for this pool to serve traffic. If the
    /// number of healthy origins falls below this number, the pool will be marked unhealthy and we
    /// will failover to the next available pool.
    pub minimum_origins: Option<u8>,
    /// The ID of the Monitor to use for health checking origins within this pool.
    pub monitor: Option<String>,
    /// The email address to send health status notifications to. This can be an individual mailbox
    /// or a mailing list.
    pub notification_email: Option<String>,
}
json_content!(CreateParams);

/// Delete Pool
///
/// https://api.cloudflare.com/#account-load-balancer-pools-delete-pool
#[derive(Debug, Clone, PartialEq)]
pub struct Delete<'a> {
    /// The Cloudflare account of this pool.
    pub account_id: &'a str,
    /// Which pool to delete.
    pub pool_id: &'a str,
}
impl Endpoint for Delete<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<DeleteResponse>;

    const METHOD: Method = Method::Delete;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/load_balancers/pools/{}",
            self.account_id, self.pool_id
        )
        .into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeleteResponse {
    pub id: String,
}
