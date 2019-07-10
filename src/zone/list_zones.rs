use super::{OrderDirection, SearchMatch};
use crate::endpoint::{Endpoint, Method};
use crate::zone::{Status, Zone};

/// List Zones
/// List, search, sort, and filter your zones
/// https://api.cloudflare.com/#zone-list-zones
pub struct ListZones<'a> {
    pub identifier: &'a str,
}
impl<'a> Endpoint<Vec<Zone>, ListZonesParams> for ListZones<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        "zones".to_string()
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ListZonesParams {
    pub name: Option<String>,
    pub status: Option<Status>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order: Option<ListZonesOrder>,
    pub direction: Option<OrderDirection>,
    #[serde(rename = "match")]
    pub search_match: Option<SearchMatch>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ListZonesOrder {
    Name,
    Status,
    Email,
}
