use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::response::ApiResult;

/// Delete Pool
/// https://api.cloudflare.com/#account-load-balancer-pools-delete-pool
#[derive(Debug)]
pub struct DeletePool<'a> {
    /// The Cloudflare account of this pool.
    pub account_identifier: &'a str,
    /// Which pool to delete.
    pub identifier: &'a str,
}

impl<'a> Endpoint<Response, (), ()> for DeletePool<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/load_balancers/pools/{}",
            self.account_identifier, self.identifier
        )
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Response {
    id: String,
}
impl ApiResult for Response {}
