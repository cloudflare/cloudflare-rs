use crate::endpoints::cfd_tunnel::TunnelConfigurationResult;
use crate::framework::endpoint::{EndpointSpec, Method};
use uuid::Uuid;

#[derive(Debug)]
pub struct GetTunnelConfiguration<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: Uuid,
}

impl<'a> EndpointSpec<TunnelConfigurationResult> for GetTunnelConfiguration<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/cfd_tunnel/{}/configurations",
            self.account_identifier, self.tunnel_id
        )
    }

    #[inline]
    fn body(&self) -> Option<String> {
        None
    }
}
