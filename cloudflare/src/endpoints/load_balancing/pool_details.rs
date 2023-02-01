use crate::endpoints::load_balancing::Pool;
use crate::framework::endpoint::{EndpointSpec, Method};

/// Pool Details
/// <https://api.cloudflare.com/#account-load-balancer-pools-pool-details>
#[derive(Debug)]
pub struct PoolDetails<'a> {
    /// The Cloudflare account of this pool.
    pub account_identifier: &'a str,
    /// Which pool to retrieve the details of.
    pub identifier: &'a str,
}

impl<'a> EndpointSpec<Pool> for PoolDetails<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/load_balancers/pools/{}",
            self.account_identifier, self.identifier
        )
    }
}
