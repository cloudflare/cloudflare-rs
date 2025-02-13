use crate::framework::endpoint::{EndpointSpec, Method};

use cloudflare_derive_macros::ApiResult;
use serde::Deserialize;

/// Delete Load Balancer
/// <https://api.cloudflare.com/#load-balancers-delete-load-balancer>
#[derive(Debug)]
pub struct DeleteLoadBalancer<'a> {
    /// The Zone to which this Load Balancer belongs.
    pub zone_identifier: &'a str,
    /// Which load balancer to delete.
    pub identifier: &'a str,
}

impl<'a> EndpointSpec<Response> for DeleteLoadBalancer<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/load_balancers/{}",
            self.zone_identifier, self.identifier
        )
    }
}

#[derive(Deserialize, Clone, Debug, ApiResult)]
pub struct Response {
    pub id: String,
}
