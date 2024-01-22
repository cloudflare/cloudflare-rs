use crate::framework::endpoint::{EndpointSpec, Method};

use cloudflare_derive_macros::ApiResult;
use serde::Deserialize;

/// Delete Pool
/// <https://api.cloudflare.com/#account-load-balancer-pools-delete-pool>
#[derive(Debug)]
pub struct DeletePool<'a> {
    /// The Cloudflare account of this pool.
    pub account_identifier: &'a str,
    /// Which pool to delete.
    pub identifier: &'a str,
}

impl<'a> EndpointSpec<Response> for DeletePool<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/load_balancers/pools/{}",
            self.account_identifier, self.identifier
        )
    }
}

#[derive(Deserialize, Clone, Debug, ApiResult)]
pub struct Response {
    pub id: String,
}
