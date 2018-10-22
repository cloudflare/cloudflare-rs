use account::Account;
use chrono::DateTime;
use chrono::offset::Utc;
use endpoint::{Endpoint, EndpointInfo};
use plan::Plan;
use reqwest::Method;
use response::{APIResponse, APIResult};
use super::HTTPAPIClient;


pub enum ZoneEndpoint<'a> {
    ZoneDetails{identifier: &'a str},
}

impl<'a> Endpoint for ZoneEndpoint<'a> {
    fn info(&self) -> EndpointInfo {
        match self {
            ZoneEndpoint::ZoneDetails{identifier} => EndpointInfo{
                method: Method::GET, 
                path: format!("zones/{}", identifier)
            }
        }
    }
}

/// https://api.cloudflare.com/#zone-properties
pub trait APIZoneClient {
    /// Zone Details (https://api.cloudflare.com/#zone-zone-details)
    fn zone_details(&self, identifier: &str) -> APIResponse<Zone>;
}

#[derive(Deserialize, Debug)]
#[serde(rename = "status")]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    Pending,
    Initializing,
    Moved,
    Deleted,
    Deactivated,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Owner {
    User {id: String, email: String},
    Organization {id: String, name: String},
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Full,
    Partial,
}

#[derive(Deserialize, Debug)]
pub struct HostingPartner {
    /// Host company name
    name: String,
    /// The host's website URL
    website: String,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    /// Maximum custom certificates that can be uploaded/used.
    custom_certificate_quota: u32,
    /// Maximum page rules that can be created.
    page_rule_quota: u32,
    /// Indicates whether wildcard DNS records can receive Cloudflare security and performance
    /// features
    wildcard_proxiable: bool,
    /// Indicates if URLs on the zone have been identified as hosting phishing content.
    phishing_detected: bool,
    /// Indicates whether the zone is allowed to be connected to multiple Railguns at once
    multiple_railguns_allowed: bool,
}


/// A Zone is a domain name along with its subdomains and other identities
/// https://api.cloudflare.com/#zone-properties
#[derive(Deserialize, Debug)]
pub struct Zone {
    /// Zone identifier tag
    id: String,
    /// The domain name
    name: String,
    /// Information about the account the zone belongs to
    account: Account,
    /// A list of beta features in which the zone is participating
    betas: Option<Vec<String>>,
    /// When the zone was created
    created_on: DateTime<Utc>,
    /// Exists only with a deactivated status and indicates the reason the zone is not resolving on
    /// the Cloudflare network.
    deactivation_reason: Option<String>,
    /// The interval (in seconds) from when development mode expires (positive integer) or last
    /// expired (negative integer) for the domain. If development mode has never been enabled, this
    /// value is 0.
    development_mode: u8,
    /// Hosting partner information, if the zone signed up via a Cloudflare hosting partner
    host: Option<HostingPartner>,
    /// Metadata about the domain.
    meta: Meta,
    /// When the zone was last modified
    modified_on: DateTime<Utc>,
    /// Cloudflare-assigned name servers. This is only populated for zones that use Cloudflare DNS
    name_servers: Vec<String>,
    /// DNS host at the time of switching to Cloudflare
    original_dnshost: Option<String>,
    /// Original name servers before moving to Cloudflare
    original_name_servers: Option<Vec<String>>,
    /// Registrar for the domain at the time of switching to Cloudflare
    original_registrar: Option<String>,
    /// Information about the owner of the zone
    owner: Owner,
    /// Indicates if the zone is only using Cloudflare DNS services. A true value means the zone
    /// will not receive security or performance benefits.
    paused: bool,
    /// Available permissions on the zone for the current user requesting the item
    permissions: Vec<String>,
    /// A zone plan
    plan: Option<Plan>,
    /// A zone plan
    plan_pending: Option<Plan>,
    /// Status of the zone
    status: Status,
    /// An array of domains used for custom name servers. This is only available for Business and
    /// Enterprise plans.
    vanity_name_servers: Vec<String>,
    /// A full zone implies that DNS is hosted with Cloudflare. A partial zone is typically a
    /// partner-hosted zone or a CNAME setup.
    #[serde(rename = "type")]
    zone_type: Type,
}

impl APIZoneClient for HTTPAPIClient {
    fn zone_details(&self, identifier: &str) -> APIResponse<Zone> {
        self.request::<Zone>(&ZoneEndpoint::ZoneDetails{identifier: identifier})
    }
}

impl APIResult for Zone {
}
