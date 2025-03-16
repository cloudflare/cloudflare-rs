use crate::endpoints::cfd_tunnel::{TunnelConfiguration, TunnelConfigurationResult};
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::Serialize;
use serde_with::serde_as;
use uuid::Uuid;

#[derive(Debug)]
pub struct UpdateTunnelConfiguration<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: Uuid,
    pub params: Params,
}

impl<'a> EndpointSpec<TunnelConfigurationResult> for UpdateTunnelConfiguration<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/cfd_tunnel/{}/configurations",
            self.account_identifier, self.tunnel_id
        )
    }

    #[inline]
    fn body(&self) -> Option<String> {
        Some(serde_json::to_string(&self.params).unwrap())
    }
}

#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Params {
    pub config: TunnelConfiguration,
}
