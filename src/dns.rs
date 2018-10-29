use chrono::DateTime;
use chrono::offset::Utc;
use endpoint::{Endpoint, EndpointInfo};
use reqwest::Method;
use response::{APIResponse, APIResult};
use std::net::{Ipv4Addr, Ipv6Addr};
use super::{HTTPAPIClient, OrderDirection, SearchMatch};
use serde_qs;


pub enum DNSRecordEndpoint<'a> {
    ListDNSRecords{zone_identifier: &'a str, params: Option<ListDNSRecordsParams>},
}

/// https://api.cloudflare.com/#dns-records-for-a-zone-properties
pub trait APIDNSRecordsClient {
    /// List DNS Records (https://api.cloudflare.com/#dns-records-for-a-zone-list-dns-records)
    fn list_dns_records(&self, zone_identifier: &str, params: Option<ListDNSRecordsParams>) -> APIResponse<Vec<DNSRecord>>;
}

impl<'a> Endpoint for DNSRecordEndpoint<'a> {
    fn info(&self) -> EndpointInfo {
        match self {
            DNSRecordEndpoint::ListDNSRecords{zone_identifier, params} => {
                let params = serde_qs::to_string(&params).unwrap();

                EndpointInfo{
                    method: Method::GET, 
                    path: format!("zones/{}/dns_records?{}", zone_identifier, params)
                }
            }
        }
    }
}

impl APIDNSRecordsClient for HTTPAPIClient {
    fn list_dns_records(&self, zone_identifier: &str, params: Option<ListDNSRecordsParams>) -> APIResponse<Vec<DNSRecord>> {
        self.request::<Vec<DNSRecord>>(&DNSRecordEndpoint::ListDNSRecords{zone_identifier: zone_identifier, params: params})
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ListDNSRecordsOrder {
    Type,
    Name,
    Content,
    TTL,
    Proxied,
}

#[derive(Serialize, Debug)]
pub struct ListDNSRecordsParams {
    record_type: Option<DNSRecordType>,
    name: Option<String>,
    page: Option<u32>,
    per_page: Option<u32>,
    order: Option<ListDNSRecordsOrder>,
    direction: Option<OrderDirection>,
    #[serde(rename = "match")]
    search_match: Option<SearchMatch>,
}

/// Extra Cloudflare-specific information about the record
#[derive(Deserialize, Debug)]
pub struct Meta {
    /// Will exist if Cloudflare automatically added this DNS record during initial setup.
    auto_added: bool,
}

/// Type of the DNS record, along with the associated value.
/// When we add support for other types (LOC/SRV/...), the `meta` field should also probably be encoded
/// here as an associated, strongly typed value.
#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum DNSRecordType {
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
    meta: Meta,
    /// Whether this record can be modified/deleted (true means it's managed by Cloudflare)
    locked: bool,
    /// DNS record name
    name: String,
    /// Time to live for DNS record. Value of 1 is 'automatic'
    ttl: u32,
    /// Zone identifier tag
    zone_id: String,
    /// When the record was last modified
    modified_on: DateTime<Utc>,
    /// When the record was created
    created_on: DateTime<Utc>,
    /// Whether this record can be modified/deleted (true means it's managed by Cloudflare)
    proxiable: bool,
    /// Type of the DNS record that also holds the record value
    #[serde(flatten)]
    content: DNSRecordType,
    /// DNS record identifier tag
    id: String,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    proxied: bool,
    /// The domain of the record
    zone_name: String,
}

impl APIResult for DNSRecord {}
impl APIResult for Vec<DNSRecord> {}
