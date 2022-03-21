use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::response::ApiResult;

/// Delete Load Balancer
/// https://api.cloudflare.com/#load-balancers-delete-load-balancer
#[derive(Debug)]
pub struct DeleteLoadBalancer<'a> {
    /// The Zone to which this Load Balancer belongs.
    pub zone_identifier: &'a str,
    /// Which load balancer to delete.
    pub identifier: &'a str,
}

impl<'a> Endpoint<Response, (), ()> for DeleteLoadBalancer<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/load_balancers/{}",
            self.zone_identifier, self.identifier
        )
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Response {
    pub id: String,
}
impl ApiResult for Response {}
