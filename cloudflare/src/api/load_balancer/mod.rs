pub mod pools;

use crate::{json_content, Endpoint, JsonResponse, Method};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoadBalancer {
    pub id: String,
    pub created_on: DateTime<Utc>,
    pub modified_on: DateTime<Utc>,
    pub description: String,
    /// The DNS hostname to associate with your Load Balancer. If this hostname already exists as a
    /// DNS record in Cloudflare's DNS, the Load Balancer will take precedence and the DNS record
    /// will not be used.
    pub name: String,
    pub enabled: bool,
    /// Time to live (TTL) of the DNS entry for the IP address returned by this load balancer. This
    /// only applies to gray-clouded (unproxied) load balancers.
    #[serde(default = "LoadBalancer::default_ttl")]
    pub ttl: u32,
    /// The pool ID to use when all other pools are detected as unhealthy.
    pub fallback_pool: LbPoolId,
    /// A list of pool IDs ordered by their failover priority. Pools defined here are used by
    /// default, or when region_pools are not configured for a given region.
    pub default_pools: Vec<LbPoolId>,
    /// A mapping of region/country codes to a list of pool IDs (ordered by their failover priority)
    /// for the given region. Any regions not explicitly defined will fall back to using
    /// default_pools.
    pub region_pools: LbPoolMapping,
    /// A mapping of Cloudflare PoP identifiers to a list of pool IDs (ordered by their failover
    /// priority) for the PoP (datacenter). Any PoPs not explicitly defined will fall back to using
    /// default_pools.
    pub pop_pools: LbPoolMapping,
    pub proxied: bool,
    pub steering_policy: SteeringPolicy,
    pub session_affinity: SessionAffinity,
    pub session_affinity_attributes: SessionAffinityAttributes,
    #[serde(default = "LoadBalancer::default_session_affinity_ttl")]
    pub session_affinity_ttl: u32,
}

impl LoadBalancer {
    fn default_ttl() -> u32 {
        30
    }
    fn default_session_affinity_ttl() -> u32 {
        5000
    }
}

type LbPoolId = String;
type LbPoolMapping = HashMap<String, Vec<LbPoolId>>;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SteeringPolicy {
    /// Empty policy maps to `Geo` if `region_pools` or `pop_pools` are used, or otherwise `Off`.
    #[serde(rename = "")]
    Nil,
    Off,
    Geo,
    Random,
    DynamicLatency,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SessionAffinity {
    /// Empty has the same behaviour as `None`.
    #[serde(rename = "")]
    Nil,
    None,
    Cookie,
    IpCookie
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SessionAffinityAttributes {
    pub samesite: SameSite,
    pub secure: Secure,
    pub drain_duration: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SameSite {
    /// `Auto` maps to `Lax` if Always Use HTTPS is set, or `None` otherwise.
    Auto,
    Lax,
    None,
    Strict,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Secure {
    /// `Auto` maps to `Always` if Always Use HTTPS is set, or `Never` otherwise.
    Auto,
    Always,
    Never
}

/// Create Load Balancer
/// https://api.cloudflare.com/#load-balancers-create-load-balancer
#[derive(Debug, Clone, PartialEq)]
pub struct Create<'a> {
    /// The Zone to which this Load Balancer shall belong.
    pub zone_id: &'a str,
    /// Parameters for the API call
    pub params: CreateParams,
}

impl Endpoint for Create<'_> {
    type Body = CreateParams;
    type Query = ();
    type Response = JsonResponse<LoadBalancer>;

    const METHOD: Method = Method::Post;

    fn path(&self) -> Cow<str> {
        format!("zones/{}/load_balancers", self.zone_id).into()
    }
    fn body(&self) -> &Self::Body {
        &self.params
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// Mandatory parameters for creating a Load Balancer.
#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateParams {
    /// A short name (tag) for the load balancer.
    /// Only alphanumeric characters, hyphens and underscores are allowed.
    /// E.g. "lb-user-facing"
    pub name: String,
    /// The list of LB Pools (by their IDs) ordered by their failover priority.
    pub default_pools: Vec<LbPoolId>,
    /// The LB Pool ID to use when all other pools are detected as unhealthy.
    pub fallback_pool: Vec<LbPoolId>,
    pub description: Option<String>,
    /// Time to live (TTL) of the DNS entry for the IP address returned by this load balancer. This
    /// only applies to gray-clouded (unproxied) load balancers.
    pub ttl: Option<u32>,
    /// A mapping of Cloudflare PoP identifiers to a list of pool IDs (ordered by their failover
    /// priority) for the PoP (datacenter). Any PoPs not explicitly defined will fall back to using
    /// default_pools.
    pub pop_pools: Option<LbPoolMapping>,
    /// A mapping of region/country codes to a list of pool IDs (ordered by their failover priority)
    /// for the given region. Any regions not explicitly defined will fall back to using
    /// default_pools.
    pub region_pools: Option<LbPoolMapping>,
    pub proxied: Option<bool>,
    pub steering_policy: Option<SteeringPolicy>,
    pub session_affinity: Option<SessionAffinity>,
    pub session_affinity_attributes: Option<SessionAffinityAttributes>,
    pub session_affinity_ttl: Option<u32>,
}
json_content!(CreateParams);

impl CreateParams {
    pub fn new(name: String) -> Self {
        Self {
            name,
            default_pools: Vec::new(),
            fallback_pool: Vec::new(),
            description: None,
            ttl: None,
            pop_pools: None,
            region_pools: None,
            proxied: None,
            steering_policy: None,
            session_affinity: None,
            session_affinity_attributes: None,
            session_affinity_ttl: None,
        }
    }
}

/// List Load Balancers
/// https://api.cloudflare.com/#load-balancers-list-load-balancers
#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    /// The Zone to list Load Balancers from.
    pub zone_id: &'a str,
}

impl Endpoint for List<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<Vec<LoadBalancer>>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!("zones/{}/load_balancers", self.zone_id).into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// Delete Load Balancer
/// https://api.cloudflare.com/#load-balancers-delete-load-balancer
#[derive(Debug, Clone, PartialEq)]
pub struct Delete<'a> {
    /// The Zone to which this Load Balancer belongs.
    pub zone_id: &'a str,
    /// Which load balancer to delete.
    pub load_balancer_id: &'a str,
}

impl Endpoint for Delete<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<DeletedId>;

    const METHOD: Method = Method::Delete;

    fn path(&self) -> Cow<str> {
        format!("zones/{}/load_balancers/{}", self.zone_id, self.load_balancer_id).into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeletedId {
    pub id: String,
}