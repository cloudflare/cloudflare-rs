use crate::framework::{
    endpoint::{serialize_query, EndpointSpec, Method},
    response::ApiResult,
};
/// <https://api.cloudflare.com/#dns-records-for-a-zone-properties>
use crate::framework::{OrderDirection, SearchMatch};
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};

/// List DNS Records
/// <https://api.cloudflare.com/#dns-records-for-a-zone-list-dns-records>
#[derive(Debug)]
pub struct ListDnsRecords<'a> {
    pub zone_identifier: &'a str,
    pub params: ListDnsRecordsParams,
}
impl<'a> EndpointSpec<Vec<DnsRecord>> for ListDnsRecords<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("zones/{}/dns_records", self.zone_identifier)
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

/// Create DNS Record
/// <https://api.cloudflare.com/#dns-records-for-a-zone-create-dns-record>
#[derive(Debug)]
pub struct CreateDnsRecord<'a> {
    pub zone_identifier: &'a str,
    pub params: CreateDnsRecordParams<'a>,
}

impl<'a> EndpointSpec<DnsRecord> for CreateDnsRecord<'a> {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/dns_records", self.zone_identifier)
    }
    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct CreateDnsRecordParams<'a> {
    /// Time to live for DNS record. Value of 1 is 'automatic'
    pub ttl: Option<u32>,
    /// Used with some records like MX and SRV to determine priority.
    /// If you do not supply a priority for an MX record, a default value of 0 will be set
    pub priority: Option<u16>,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    pub proxied: Option<bool>,
    /// DNS record name
    pub name: &'a str,
    /// Type of the DNS record that also holds the record value
    #[serde(flatten)]
    pub content: DnsContent,
}

/// Delete DNS Record
/// <https://api.cloudflare.com/#dns-records-for-a-zone-delete-dns-record>
#[derive(Debug)]
pub struct DeleteDnsRecord<'a> {
    pub zone_identifier: &'a str,
    pub identifier: &'a str,
}
impl<'a> EndpointSpec<DeleteDnsRecordResponse> for DeleteDnsRecord<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/dns_records/{}",
            self.zone_identifier, self.identifier
        )
    }
}

/// Update DNS Record
/// <https://api.cloudflare.com/#dns-records-for-a-zone-update-dns-record>
#[derive(Debug)]
pub struct UpdateDnsRecord<'a> {
    pub zone_identifier: &'a str,
    pub identifier: &'a str,
    pub params: UpdateDnsRecordParams<'a>,
}

impl<'a> EndpointSpec<DnsRecord> for UpdateDnsRecord<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/dns_records/{}",
            self.zone_identifier, self.identifier
        )
    }
    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct UpdateDnsRecordParams<'a> {
    /// Time to live for DNS record. Value of 1 is 'automatic'
    pub ttl: Option<u32>,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    pub proxied: Option<bool>,
    /// DNS record name
    pub name: &'a str,
    /// Type of the DNS record that also holds the record value
    #[serde(flatten)]
    pub content: DnsContent,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ListDnsRecordsOrder {
    Type,
    Name,
    Content,
    Ttl,
    Proxied,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListDnsRecordsParams {
    #[serde(flatten)]
    pub record_type: Option<DnsContent>,
    pub name: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order: Option<ListDnsRecordsOrder>,
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
#[allow(clippy::upper_case_acronyms)]
pub enum DnsContent {
    A { content: Ipv4Addr },
    AAAA { content: Ipv6Addr },
    CNAME { content: String },
    NS { content: String },
    MX { content: String, priority: u16 },
    TXT { content: String },
    SRV { content: String },
}

#[derive(Deserialize, Debug)]
pub struct DeleteDnsRecordResponse {
    /// DNS record identifier tag
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct DnsRecord {
    /// Extra Cloudflare-specific information about the record
    pub meta: Meta,
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
    pub content: DnsContent,
    /// DNS record identifier tag
    pub id: String,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    pub proxied: bool,
    /// The domain of the record
    pub zone_name: String,
}

impl ApiResult for DnsRecord {}
impl ApiResult for Vec<DnsRecord> {}
impl ApiResult for DeleteDnsRecordResponse {}
