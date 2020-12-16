use crate::endpoints::load_balancing::Pool;
use crate::framework::endpoint::{Endpoint, Method};

/// Pool Details
/// https://api.cloudflare.com/#account-load-balancer-pools-pool-details
#[derive(Debug)]
pub struct PoolDetails<'a> {
    /// The Cloudflare account of this pool.
    pub account_identifier: &'a str,
    /// Which pool to retrieve the details of.
    pub identifier: &'a str,
}

impl<'a> Endpoint<Pool, (), ()> for PoolDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/load_balancers/pools/{}",
            self.account_identifier, self.identifier
        )
    }
}
