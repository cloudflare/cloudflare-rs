use crate::api::accounts::Account;
use crate::{json_content, Endpoint, JsonResponse, Method, OrderDirection, SearchMatch};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// List Zones
/// List, search, sort, and filter your zones
/// https://api.cloudflare.com/#zone-list-zones
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct List {
    pub name: Option<String>,
    pub status: Option<Status>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order: Option<ListOrder>,
    pub direction: Option<OrderDirection>,
    #[serde(rename = "match")]
    pub search_match: Option<SearchMatch>,
}

impl Endpoint for List {
    type Body = ();
    type Query = Self;
    type Response = JsonResponse<Vec<Zone>>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        "zones".into()
    }
    fn query(&self) -> &Self::Query {
        self
    }
    fn body(&self) -> &Self::Body {
        &()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename = "status", rename_all = "lowercase")]
pub enum Status {
    Active,
    Pending,
    Initializing,
    Moved,
    Deleted,
    Deactivated,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListOrder {
    Name,
    Status,
    Email,
}

/// Zone Details
/// https://api.cloudflare.com/#zone-zone-details
#[derive(Debug, Clone, PartialEq)]
pub struct Get<'a> {
    pub zone_id: &'a str,
}
impl Endpoint for Get<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<Zone>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!("zones/{}", self.zone_id).into()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
}

/// Add Zone
/// https://api.cloudflare.com/#zone-create-zone
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Create {
    pub name: String,
    pub account: String,
    pub jump_start: Option<bool>,
    #[serde(rename = "type")]
    pub zone_type: Option<Type>,
}
json_content!(Create);
impl Endpoint for Create {
    type Body = Self;
    type Query = ();
    type Response = JsonResponse<Zone>;

    const METHOD: Method = Method::Post;

    fn path(&self) -> Cow<str> {
        "zones".into()
    }
    fn body(&self) -> &Self::Body {
        self
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// A Zone is a domain name along with its subdomains and other identities
/// https://api.cloudflare.com/#zone-properties
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Zone {
    /// Zone identifier tag
    pub id: String,
    /// The domain name
    pub name: String,
    /// Information about the account the zone belongs to
    pub account: Account,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Owner {
    User { id: String, email: String },
    Organization { id: String, name: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Full,
    Partial,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HostingPartner {
    /// Host company name
    pub name: String,
    /// The host's website URL
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Meta {
    /// Maximum custom certificates that can be uploaded/used.
    pub custom_certificate_quota: u32,
    /// Maximum page rules that can be created.
    pub page_rule_quota: u32,
    /// Indicates whether wildcard DNS records can receive Cloudflare security and performance
    /// features
    pub wildcard_proxiable: bool,
    /// Indicates if URLs on the zone have been identified as hosting phishing content.
    pub phishing_detected: bool,
    /// Indicates whether the zone is allowed to be connected to multiple Railguns at once
    pub multiple_railguns_allowed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
/// Free plans won't have a Frequency, so most responses should accept Option instead.
pub enum Frequency {
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Plan {
    /// Plan identifier tag
    pub id: String,
    /// The plan name
    pub name: String,
    /// The price of the subscription that will be billed, in US dollars
    pub price: f64,
    /// The monetary unit in which pricing information is displayed
    pub currency: String,
    /// The frequency at which you will be billed for this plan
    pub frequency: Option<Frequency>,
    /// A 'friendly' identifier to indicate to the UI what plan the object is
    pub legacy_id: String,
    /// If the zone is subscribed to this plan
    pub is_subscribed: bool,
    /// If the zone is allowed to subscribe to this plan
    pub can_subscribe: bool,
}
