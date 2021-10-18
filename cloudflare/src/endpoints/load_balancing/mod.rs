pub mod create_lb;
pub mod create_pool;
pub mod delete_lb;
pub mod delete_pool;
pub mod list_lb;
pub mod pool_details;

use crate::framework::response::ApiResult;
use chrono::offset::Utc;
use chrono::DateTime;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::net::IpAddr;

#[derive(Eq, PartialEq, Deserialize, Serialize, Clone, Debug)]
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

#[derive(Eq, PartialEq, Deserialize, Serialize, Clone, Debug)]
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

#[derive(Eq, PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SessionAffinity {
    /// Empty has the same behaviour as `None`.
    #[serde(rename = "")]
    Nil,
    None,
    Cookie,
    IpCookie,
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Clone, Debug)]
pub struct SessionAffinityAttributes {
    pub samesite: SameSite,
    pub secure: Secure,
    pub drain_duration: u32,
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Clone, Debug)]
pub enum SameSite {
    /// `Auto` maps to `Lax` if Always Use HTTPS is set, or `None` otherwise.
    Auto,
    Lax,
    None,
    Strict,
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Clone, Debug)]
pub enum Secure {
    /// `Auto` maps to `Always` if Always Use HTTPS is set, or `Never` otherwise.
    Auto,
    Always,
    Never,
}

impl ApiResult for LoadBalancer {}

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
#[derive(Eq, PartialEq, Deserialize, Serialize, Clone, Debug)]
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
    pub origins: HashSet<Origin>,
    /// The email address to send health status notifications to. This can be an individual mailbox
    /// or a mailing list.
    pub notification_email: String,
}

/// An origin represents something that can serve user requests. Usually a machine, maybe an ELB.
/// Origins with similar latency functions (e.g. origins in the same data center or region) might be
/// in the same pool.
#[derive(Deserialize, Serialize, Clone, Debug)]
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

// f64 doesn't impl Eq or Hash, so we need some custom implementations.
impl PartialEq for Origin {
    fn eq(&self, other: &Self) -> bool {
        let diff_is_small = (self.weight - other.weight).abs() < 0.01;
        self.name == other.name
            && self.address == other.address
            && self.enabled == other.enabled
            && diff_is_small
    }
}
impl Eq for Origin {}

impl Hash for Origin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.address.hash(state);
        self.enabled.hash(state);
        self.weight.to_bits().hash(state);
    }
}

impl ApiResult for Origin {}
impl ApiResult for Pool {}
