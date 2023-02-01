use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::framework::endpoint::{serialize_query, EndpointSpec, Method};

use super::Tunnel;

/// List/search tunnels in an account.
/// <https://api.cloudflare.com/#argo-tunnel-list-argo-tunnels>
#[derive(Debug)]
pub struct ListTunnels<'a> {
    pub account_identifier: &'a str,
    pub params: Params,
}

impl<'a> EndpointSpec<Vec<Tunnel>> for ListTunnels<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("accounts/{}/tunnels", self.account_identifier)
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

/// Params for filtering listed tunnels
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct Params {
    pub name: Option<String>,
    pub uuid: Option<String>,
    pub is_deleted: Option<bool>,
    pub existed_at: Option<DateTime<Utc>>,
    pub name_prefix: Option<String>,
    pub was_inactive_at: Option<DateTime<Utc>>,
    pub exclude_prefix: Option<String>,
    #[serde(flatten)]
    pub pagination_params: Option<PaginationParams>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PaginationParams {
    pub page: u64,
    pub per_page: u64,
}
