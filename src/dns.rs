/// https://api.cloudflare.com/#dns-records-for-a-zone-properties

use chrono::DateTime;
use chrono::offset::Utc;
use endpoint::{Endpoint, Method};
use response::APIResult;
use std::net::{Ipv4Addr, Ipv6Addr};
use super::{OrderDirection, SearchMatch};


/// List DNS Records (https://api.cloudflare.com/#dns-records-for-a-zone-list-dns-records)
pub struct ListDNSRecords<'a> { pub zone_identifier: &'a str, pub params: ListDNSRecordsParams }
impl<'a> Endpoint<Vec<DNSRecord>, ListDNSRecordsParams, ListDNSRecordsParams> for ListDNSRecords<'a> {
    fn method(&self) -> Method { Method::Get }
    fn path(&self) -> String { format!("zones/{}/dns_records", self.zone_identifier) }
    fn query(&self) -> Option<ListDNSRecordsParams> { Some(self.params.clone()) }
}

pub struct DeleteDNSRecord<'a> { pub zone_identifier: &'a str, pub identifier: &'a str }
impl<'a> Endpoint<()> for DeleteDNSRecord<'a> {
    fn method(&self) -> Method { Method::Delete }
    fn path(&self) -> String { format!("zones/{}/dns_records/{}", self.zone_identifier, self.identifier) }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ListDNSRecordsOrder {
    Type,
    Name,
    Content,
    TTL,
    Proxied,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ListDNSRecordsParams {
    pub record_type: Option<DNSContent>,
    pub name: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order: Option<ListDNSRecordsOrder>,
    pub direction: Option<OrderDirection>,
    #[serde(rename = "match")]
    pub search_match: Option<SearchMatch>,
}

/// Extra Cloudflare-specific information about the record
#[derive(Deserialize, Debug)]
pub struct Meta {
    /// Will exist if Cloudflare automatically added this DNS record during initial setup.
    pub auto_added: bool,
}

/// Type of the DNS record, along with the associated value.
/// When we add support for other types (LOC/SRV/...), the `meta` field should also probably be encoded
/// here as an associated, strongly typed value.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum DNSContent {
    A{content: Ipv4Addr},
    AAAA{content: Ipv6Addr},
    CNAME{content: String},
    NS{content: String},
    MX{content: String, priority: u16},
    TXT{content: String},
}

#[derive(Deserialize, Debug)]
pub struct DNSRecord {
    /// Extra Cloudflare-specific information about the record
    pub meta: Meta,
    /// Whether this record can be modified/deleted (true means it's managed by Cloudflare)
    pub locked: bool,
    /// DNS record name
    pub name: String,
    /// Time to live for DNS record. Value of 1 is 'automatic'
    pub ttl: u32,
    /// Zone identifier tag
    pub zone_id: String,
    /// When the record was last modified
    pub modified_on: DateTime<Utc>,
    /// When the record was created
    pub created_on: DateTime<Utc>,
    /// Whether this record can be modified/deleted (true means it's managed by Cloudflare)
    pub proxiable: bool,
    /// Type of the DNS record that also holds the record value
    #[serde(flatten)]
    pub content: DNSContent,
    /// DNS record identifier tag
    pub id: String,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    pub proxied: bool,
    /// The domain of the record
    pub zone_name: String,
}

impl APIResult for DNSRecord {}
impl APIResult for Vec<DNSRecord> {}
