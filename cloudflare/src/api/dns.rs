use crate::{json_content, Endpoint, JsonResponse, Method, OrderDirection, SearchMatch};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::net::{Ipv4Addr, Ipv6Addr};

/// List DNS Records
/// https://api.cloudflare.com/#dns-records-for-a-zone-list-dns-records
#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    pub zone_id: &'a str,
    pub params: ListParams,
}
impl Endpoint for List<'_> {
    type Body = ();
    type Query = ListParams;
    type Response = JsonResponse<Vec<DnsRecord>>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!("zones/{}/dns_records", self.zone_id).into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &self.params
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct ListParams {
    #[serde(flatten)]
    pub record_type: Option<DnsContent>,
    pub name: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order: Option<ListOrder>,
    pub direction: Option<OrderDirection>,
    #[serde(rename = "match")]
    pub search_match: Option<SearchMatch>,
}

/// Create DNS Record
///
/// https://api.cloudflare.com/#dns-records-for-a-zone-create-dns-record
#[derive(Debug, Clone, PartialEq)]
pub struct Create<'a> {
    pub zone_id: &'a str,
    pub params: CreateParams,
}

impl Endpoint for Create<'_> {
    type Body = CreateParams;
    type Query = ();
    type Response = JsonResponse<DnsRecord>;

    const METHOD: Method = Method::Post;

    fn path(&self) -> Cow<str> {
        format!("zones/{}/dns_records", self.zone_id).into()
    }
    fn body(&self) -> &CreateParams {
        &self.params
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateParams {
    /// Time to live for DNS record. Value of 1 is 'automatic'
    pub ttl: Option<u32>,
    /// Used with some records like MX and SRV to determine priority.
    /// If you do not supply a priority for an MX record, a default value of 0 will be set
    pub priority: Option<u16>,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    pub proxied: Option<bool>,
    /// DNS record name
    pub name: String,
    /// Type of the DNS record that also holds the record value
    #[serde(flatten)]
    pub content: DnsContent,
}
json_content!(CreateParams);

/// Delete DNS Record
///
/// https://api.cloudflare.com/#dns-records-for-a-zone-delete-dns-record
#[derive(Debug, Clone, PartialEq)]
pub struct Delete<'a> {
    pub zone_id: &'a str,
    pub record_id: &'a str,
}
impl Endpoint for Delete<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<DeleteResponse>;

    const METHOD: Method = Method::Delete;

    fn path(&self) -> Cow<str> {
        format!("zones/{}/dns_records/{}", self.zone_id, self.record_id).into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteResponse {
    /// DNS record identifier tag
    pub id: String,
}

/// Update DNS Record
/// https://api.cloudflare.com/#dns-records-for-a-zone-update-dns-record
#[derive(Debug, Clone, PartialEq)]
pub struct Update<'a> {
    pub zone_id: &'a str,
    pub record_id: &'a str,
    pub params: UpdateParams,
}

impl Endpoint for Update<'_> {
    type Body = UpdateParams;
    type Query = ();
    type Response = JsonResponse<DnsRecord>;

    const METHOD: Method = Method::Put;

    fn path(&self) -> Cow<str> {
        format!("zones/{}/dns_records/{}", self.zone_id, self.record_id).into()
    }
    fn body(&self) -> &Self::Body {
        &self.params
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}
json_content!(UpdateParams);

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateParams {
    /// Time to live for DNS record. Value of 1 is 'automatic'
    pub ttl: Option<u32>,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    pub proxied: Option<bool>,
    /// DNS record name
    pub name: String,
    /// Type of the DNS record that also holds the record value
    #[serde(flatten)]
    pub content: DnsContent,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListOrder {
    Type,
    Name,
    Content,
    Ttl,
    Proxied,
}

/// Extra Cloudflare-specific information about the record
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Meta {
    /// Will exist if Cloudflare automatically added this DNS record during initial setup.
    pub auto_added: bool,
}

/// Type of the DNS record, along with the associated value.
/// When we add support for other types (LOC/SRV/...), the `meta` field should also probably be encoded
/// here as an associated, strongly typed value.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DnsContent {
    A { content: Ipv4Addr },
    AAAA { content: Ipv6Addr },
    CNAME { content: String },
    NS { content: String },
    MX { content: String, priority: u16 },
    TXT { content: String },
    SRV { content: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DnsRecord {
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
    pub content: DnsContent,
    /// DNS record identifier tag
    pub id: String,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    pub proxied: bool,
    /// The domain of the record
    pub zone_name: String,
}
