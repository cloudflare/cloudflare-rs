use crate::endpoints::load_balancing::LoadBalancer;
use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::response::ApiResult;

/// List Load Balancers
/// https://api.cloudflare.com/#load-balancers-list-load-balancers
#[derive(Debug)]
pub struct ListLoadBalancers<'a> {
    /// The Zone to list Load Balancers from.
    pub zone_identifier: &'a str,
}

impl<'a> Endpoint<Vec<LoadBalancer>, ()> for ListLoadBalancers<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("zones/{}/load_balancers", self.zone_identifier)
    }
}

impl ApiResult for Vec<LoadBalancer> {}
