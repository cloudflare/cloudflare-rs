use crate::endpoints::load_balancing::Pool;
use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Pool Details
/// <https://api.cloudflare.com/#account-load-balancer-pools-pool-details>
#[derive(Debug)]
pub struct PoolDetails<'a> {
    /// The Cloudflare account of this pool.
    pub account_identifier: &'a str,
    /// Which pool to retrieve the details of.
    pub identifier: &'a str,
}

impl EndpointSpec for PoolDetails<'_> {
    type JsonResponse = Pool;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

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
