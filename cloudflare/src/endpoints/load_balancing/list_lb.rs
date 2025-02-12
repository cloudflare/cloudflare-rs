use crate::endpoints::load_balancing::LoadBalancer;
use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::{ApiResult, ApiSuccess};

/// List Load Balancers
/// <https://api.cloudflare.com/#load-balancers-list-load-balancers>
#[derive(Debug)]
pub struct ListLoadBalancers<'a> {
    /// The Zone to list Load Balancers from.
    pub zone_identifier: &'a str,
}

impl EndpointSpec for ListLoadBalancers<'_> {
    type JsonResponse = Vec<LoadBalancer>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("zones/{}/load_balancers", self.zone_identifier)
    }
}

impl ApiResult for Vec<LoadBalancer> {}
