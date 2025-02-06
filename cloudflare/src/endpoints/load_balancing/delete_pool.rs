use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::{ApiResult, ApiSuccess};

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

impl<'a> EndpointSpec for DeletePool<'a> {
    type JsonResponse = Response;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

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

#[derive(Deserialize, Clone, Debug)]
pub struct Response {
    pub id: String,
}
impl ApiResult for Response {}
