use serde_with::serde_as;
use time::OffsetDateTime;

use crate::framework::endpoint::{Endpoint, Method};

use super::Tunnel;

/// List/search tunnels in an account.
/// https://api.cloudflare.com/#argo-tunnel-list-argo-tunnels
#[derive(Debug)]
pub struct ListTunnels<'a> {
    pub account_identifier: &'a str,
    pub params: Params,
}

impl<'a> Endpoint<Vec<Tunnel>, Params> for ListTunnels<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("accounts/{}/tunnels", self.account_identifier)
    }
    fn query(&self) -> Option<Params> {
        Some(self.params.clone())
    }
}

/// Params for filtering listed tunnels
#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct Params {
    pub name: Option<String>,
    pub uuid: Option<String>,
    pub is_deleted: Option<bool>,
    #[serde_as(as = "Option<time::format_description::well_known::Rfc3339>")]
    pub existed_at: Option<OffsetDateTime>,
    pub name_prefix: Option<String>,
    #[serde_as(as = "Option<time::format_description::well_known::Rfc3339>")]
    pub was_inactive_at: Option<OffsetDateTime>,
    pub exclude_prefix: Option<String>,
    #[serde(flatten)]
    pub pagination_params: Option<PaginationParams>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PaginationParams {
    pub page: u64,
    pub per_page: u64,
}
