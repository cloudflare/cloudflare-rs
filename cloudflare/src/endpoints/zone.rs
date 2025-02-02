use crate::endpoints::{account::AccountDetails, plan::Plan};
use crate::framework::endpoint::serialize_query;
use crate::framework::{
    endpoint::{EndpointSpec, Method},
    response::ApiResult,
};
use crate::framework::{OrderDirection, SearchMatch};
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

/// List Zones
/// List, search, sort, and filter your zones
/// <https://api.cloudflare.com/#zone-list-zones>
#[derive(Debug)]
pub struct ListZones {
    pub params: ListZonesParams,
}

impl EndpointSpec<Vec<Zone>> for ListZones {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "zones".to_string()
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

/// Zone Details
/// <https://api.cloudflare.com/#zone-zone-details>
#[derive(Debug)]
pub struct ZoneDetails<'a> {
    pub identifier: &'a str,
}
impl<'a> EndpointSpec<Zone> for ZoneDetails<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("zones/{}", self.identifier)
    }
}

/// Add Zone
/// <https://api.cloudflare.com/#zone-create-zone>
pub struct CreateZone<'a> {
    pub params: CreateZoneParams<'a>,
}
impl<'a> EndpointSpec<()> for CreateZone<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "zones".to_string()
    }

    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct CreateZoneParams<'a> {
    pub name: &'a str,
    pub account: &'a str,
    pub jump_start: Option<bool>,
    #[serde(rename = "type")]
    pub zone_type: Option<Type>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "status", rename_all = "lowercase")]
pub enum Status {
    Active,
    Pending,
    Initializing,
    Moved,
    Deleted,
    Deactivated,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Owner {
    User { id: String, email: String },
    Organization { id: String, name: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Full,
    Partial,
}

#[derive(Deserialize, Debug)]
pub struct HostingPartner {
    /// Host company name
    pub name: String,
    /// The host's website URL
    pub website: String,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    /// Maximum custom certificates that can be uploaded/used.
    pub custom_certificate_quota: u32,
    /// Maximum page rules that can be created.
    pub page_rule_quota: u32,
    /// Indicates if URLs on the zone have been identified as hosting phishing content.
    pub phishing_detected: bool,
}

/// A Zone is a domain name along with its subdomains and other identities
/// <https://api.cloudflare.com/#zone-properties>
#[derive(Deserialize, Debug)]
pub struct Zone {
    /// Zone identifier tag
    pub id: String,
    /// The domain name
    pub name: String,
    /// Information about the account the zone belongs to
    pub account: AccountDetails,
    /// A list of beta features in which the zone is participating
    pub betas: Option<Vec<String>>,
    /// When the zone was created
    pub created_on: DateTime<Utc>,
    /// Exists only with a deactivated status and indicates the reason the zone is not resolving on
    /// the Cloudflare network.
    pub deactivation_reason: Option<String>,
    /// The interval (in seconds) from when development mode expires (positive integer) or last
    /// expired (negative integer) for the domain. If development mode has never been enabled, this
    /// value is 0.
    pub development_mode: i32,
    /// Hosting partner information, if the zone signed up via a Cloudflare hosting partner
    pub host: Option<HostingPartner>,
    /// Metadata about the domain.
    pub meta: Meta,
    /// When the zone was last modified
    pub modified_on: DateTime<Utc>,
    /// Cloudflare-assigned name servers. This is only populated for zones that use Cloudflare DNS
    pub name_servers: Vec<String>,
    /// DNS host at the time of switching to Cloudflare
    pub original_dnshost: Option<String>,
    /// Original name servers before moving to Cloudflare
    pub original_name_servers: Option<Vec<String>>,
    /// Registrar for the domain at the time of switching to Cloudflare
    pub original_registrar: Option<String>,
    /// Information about the owner of the zone
    pub owner: Owner,
    /// Indicates if the zone is only using Cloudflare DNS services. A true value means the zone
    /// will not receive security or performance benefits.
    pub paused: bool,
    /// Available permissions on the zone for the current user requesting the item
    pub permissions: Vec<String>,
    /// A zone plan
    pub plan: Option<Plan>,
    /// A zone plan
    pub plan_pending: Option<Plan>,
    /// Status of the zone
    pub status: Status,
    /// An array of domains used for custom name servers. This is only available for Business and
    /// Enterprise plans.
    pub vanity_name_servers: Option<Vec<String>>,
    /// A full zone implies that DNS is hosted with Cloudflare. A partial zone is typically a
    /// partner-hosted zone or a CNAME setup.
    #[serde(rename = "type")]
    pub zone_type: Type,
}

// TODO: This should probably be a derive macro
impl ApiResult for Zone {}
impl ApiResult for Vec<Zone> {}
