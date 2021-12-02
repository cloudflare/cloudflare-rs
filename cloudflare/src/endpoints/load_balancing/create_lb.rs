use crate::endpoints::load_balancing::{
    LbPoolId, LbPoolMapping, LoadBalancer, SessionAffinity, SessionAffinityAttributes,
    SteeringPolicy,
};
use crate::framework::endpoint::{Endpoint, Method};

/// Create Load Balancer
/// https://api.cloudflare.com/#load-balancers-create-load-balancer
#[derive(Debug)]
pub struct CreateLoadBalancer<'a> {
    /// The Zone to which this Load Balancer shall belong.
    pub zone_identifier: &'a str,
    /// Optional parameters for the API call
    pub params: Params<'a>,
}

/// Mandatory parameters for creating a Load Balancer.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Params<'a> {
    /// A short name (tag) for the load balancer.
    /// Only alphanumeric characters, hyphens and underscores are allowed.
    /// E.g. "lb-user-facing"
    pub name: &'a str,
    /// The list of LB Pools (by their IDs) ordered by their failover priority.
    pub default_pools: &'a [LbPoolId],
    /// The LB Pool ID to use when all other pools are detected as unhealthy.
    pub fallback_pool: &'a LbPoolId,
    #[serde(flatten)]
    pub optional_params: Option<OptionalParams<'a>>,
}

/// Optional parameters for creating a Load Balancer.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct OptionalParams<'a> {
    pub description: Option<&'a str>,
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

impl<'a> Endpoint<LoadBalancer, (), Params<'a>> for CreateLoadBalancer<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("zones/{}/load_balancers", self.zone_identifier)
    }
    fn body(&self) -> Option<Params<'a>> {
        Some(self.params.clone())
    }
}
