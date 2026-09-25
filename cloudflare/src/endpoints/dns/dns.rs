use crate::framework::endpoint::{serialize_query, EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
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
impl EndpointSpec for ListDnsRecords<'_> {
    type JsonResponse = Vec<DnsRecord>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

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

impl EndpointSpec for CreateDnsRecord<'_> {
    type JsonResponse = DnsRecord;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/dns_records", self.zone_identifier)
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
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
impl EndpointSpec for DeleteDnsRecord<'_> {
    type JsonResponse = DeleteDnsRecordResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

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

impl EndpointSpec for UpdateDnsRecord<'_> {
    type JsonResponse = DnsRecord;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

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
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
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
    /// Comments or notes about the DNS record
    pub comment: Option<&'a str>,
    /// Custom tags for the DNS record. This field has no effect on DNS responses
    pub tags: Option<&'a [String]>,
}

/// Patch DNS Record: change only the fields that are set.
/// <https://developers.cloudflare.com/api/resources/dns/subresources/records/methods/edit/>
#[derive(Debug)]
pub struct PatchDnsRecord<'a> {
    pub zone_identifier: &'a str,
    pub identifier: &'a str,
    pub params: PatchDnsRecordParams<'a>,
}

impl EndpointSpec for PatchDnsRecord<'_> {
    type JsonResponse = DnsRecord;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/dns_records/{}",
            self.zone_identifier, self.identifier
        )
    }
    #[inline]
    fn body(&self) -> Option<RequestBody<'_>> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Unset fields are omitted from the request and left unchanged.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct PatchDnsRecordParams<'a> {
    /// Time to live for DNS record. Value of 1 is 'automatic'
    pub ttl: Option<u32>,
    /// Whether the record is receiving the performance and security benefits of Cloudflare
    pub proxied: Option<bool>,
    /// DNS record name
    pub name: Option<&'a str>,
    /// Type of the DNS record together with its new value
    #[serde(flatten)]
    pub content: Option<DnsContent>,
    /// Comments or notes about the DNS record
    pub comment: Option<&'a str>,
    /// Custom tags for the DNS record. This field has no effect on DNS responses
    pub tags: Option<&'a [String]>,
}

/// DNS Record Details
/// <https://developers.cloudflare.com/api/resources/dns/subresources/records/methods/get/>
#[derive(Debug)]
pub struct DnsRecordDetails<'a> {
    pub zone_identifier: &'a str,
    pub identifier: &'a str,
}

impl EndpointSpec for DnsRecordDetails<'_> {
    type JsonResponse = DnsRecord;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/dns_records/{}",
            self.zone_identifier, self.identifier
        )
    }
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
    /// Filters on type and exact content together. To filter on type alone,
    /// use `type_filter` instead; setting both sends `type` twice.
    #[serde(flatten)]
    pub record_type: Option<DnsContent>,
    /// Filters on record type alone, e.g. `"MX"`.
    #[serde(rename = "type")]
    pub type_filter: Option<String>,
    pub name: Option<String>,
    #[serde(flatten)]
    pub content: Option<ListDnsRecordsParamsContent>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order: Option<ListDnsRecordsOrder>,
    pub direction: Option<OrderDirection>,
    #[serde(rename = "match")]
    pub search_match: Option<SearchMatch>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListDnsRecordsParamsContent {
    #[serde(rename = "content.contains")]
    pub contains: Option<String>,
    #[serde(rename = "content.startswith")]
    pub starts_with: Option<String>,
    #[serde(rename = "content.endswith")]
    pub ends_with: Option<String>,
    #[serde(rename = "content.exact")]
    pub exact: Option<String>,
}

/// Extra Cloudflare-specific information about the record
#[derive(Deserialize, Debug, Default)]
pub struct Meta {}

/// Type of the DNS record, along with the associated value.
///
/// Record types without a dedicated variant (CAA, HTTPS, DS, TLSA, ...)
/// decode as [`DnsContent::Other`] rather than failing, so listing a zone
/// never breaks on a type this crate does not model yet. Their structured
/// fields are in [`DnsRecord::data`].
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[allow(clippy::upper_case_acronyms)]
pub enum DnsContent {
    A {
        content: Ipv4Addr,
    },
    AAAA {
        content: Ipv6Addr,
    },
    CNAME {
        content: String,
    },
    NS {
        content: String,
    },
    MX {
        content: String,
        priority: u16,
    },
    PTR {
        content: String,
    },
    TXT {
        content: String,
    },
    SRV {
        content: String,
    },
    /// Any other record type, with the type name as the API reports it.
    #[serde(untagged)]
    Other {
        #[serde(rename = "type")]
        record_type: String,
        #[serde(default)]
        content: String,
    },
}

impl DnsContent {
    /// The record type name, e.g. `"A"` or `"HTTPS"`.
    pub fn record_type(&self) -> &str {
        match self {
            DnsContent::A { .. } => "A",
            DnsContent::AAAA { .. } => "AAAA",
            DnsContent::CNAME { .. } => "CNAME",
            DnsContent::NS { .. } => "NS",
            DnsContent::MX { .. } => "MX",
            DnsContent::PTR { .. } => "PTR",
            DnsContent::TXT { .. } => "TXT",
            DnsContent::SRV { .. } => "SRV",
            DnsContent::Other { record_type, .. } => record_type,
        }
    }

    /// The record content as a string, as the API reports it.
    pub fn content(&self) -> String {
        match self {
            DnsContent::A { content } => content.to_string(),
            DnsContent::AAAA { content } => content.to_string(),
            DnsContent::CNAME { content }
            | DnsContent::NS { content }
            | DnsContent::MX { content, .. }
            | DnsContent::PTR { content }
            | DnsContent::TXT { content }
            | DnsContent::SRV { content }
            | DnsContent::Other { content, .. } => content.clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct DeleteDnsRecordResponse {
    /// DNS record identifier tag
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct DnsRecord {
    /// Extra Cloudflare-specific information about the record
    #[serde(default)]
    pub meta: Meta,
    /// DNS record name
    pub name: String,
    /// Time to live for DNS record. Value of 1 is 'automatic'
    pub ttl: u32,
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
    /// Comments or notes about the DNS record
    pub comment: Option<String>,
    /// Custom tags for the DNS record. This field has no effect on DNS responses
    #[serde(default)]
    pub tags: Vec<String>,
    /// Structured fields for record types that have them (SRV, CAA, HTTPS, ...)
    pub data: Option<serde_json::Value>,
}

impl ApiResult for DnsRecord {}
impl ApiResult for Vec<DnsRecord> {}
impl ApiResult for DeleteDnsRecordResponse {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn record_json(record_type: &str, content: &str) -> serde_json::Value {
        json!({
            "id": "372e67954025e0ba6aaa6d586b9e0b59",
            "name": "example.com",
            "type": record_type,
            "content": content,
            "proxiable": false,
            "proxied": false,
            "ttl": 1,
            "meta": {},
            "created_on": "2024-01-01T00:00:00Z",
            "modified_on": "2024-01-01T00:00:00Z"
        })
    }

    #[test]
    fn known_types_decode_to_their_variant() {
        let a: DnsRecord = serde_json::from_value(record_json("A", "192.0.2.1")).unwrap();
        assert!(matches!(a.content, DnsContent::A { .. }));

        let mut mx = record_json("MX", "mx.example.com");
        mx["priority"] = json!(10);
        let mx: DnsRecord = serde_json::from_value(mx).unwrap();
        assert!(matches!(mx.content, DnsContent::MX { priority: 10, .. }));
    }

    #[test]
    fn caa_decodes_as_other() {
        let mut caa = record_json("CAA", "0 issue \"letsencrypt.org\"");
        caa["data"] = json!({ "flags": 0, "tag": "issue", "value": "letsencrypt.org" });
        let record: DnsRecord = serde_json::from_value(caa).unwrap();
        assert_eq!(record.content.record_type(), "CAA");
        assert_eq!(record.data.unwrap()["tag"], "issue");
    }

    #[test]
    fn unmodelled_types_decode_as_other() {
        let mut https = record_json("HTTPS", "1 . alpn=\"h2\"");
        https["data"] = json!({ "priority": 1, "target": ".", "value": "alpn=\"h2\"" });
        https["tags"] = json!(["owner:web"]);
        https["comment"] = json!("svcb");
        let record: DnsRecord = serde_json::from_value(https).unwrap();

        assert_eq!(record.content.record_type(), "HTTPS");
        assert_eq!(record.content.content(), "1 . alpn=\"h2\"");
        assert_eq!(record.data.unwrap()["priority"], 1);
        assert_eq!(record.tags, ["owner:web"]);
        assert_eq!(record.comment.as_deref(), Some("svcb"));
    }

    #[test]
    fn meta_may_be_absent() {
        let mut a = record_json("A", "192.0.2.1");
        a.as_object_mut().unwrap().remove("meta");
        let record: DnsRecord = serde_json::from_value(a).unwrap();
        assert_eq!(record.content.content(), "192.0.2.1");
    }

    #[test]
    fn other_serializes_type_and_content() {
        let content = DnsContent::Other {
            record_type: "PTR".into(),
            content: "host.example.com".into(),
        };
        assert_eq!(
            serde_json::to_value(&content).unwrap(),
            json!({ "type": "PTR", "content": "host.example.com" })
        );
    }

    #[test]
    fn patch_sends_only_set_fields() {
        let params = PatchDnsRecordParams {
            proxied: Some(false),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&params).unwrap(),
            json!({ "proxied": false })
        );

        let params = PatchDnsRecordParams {
            content: Some(DnsContent::A {
                content: "192.0.2.2".parse().unwrap(),
            }),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&params).unwrap(),
            json!({ "type": "A", "content": "192.0.2.2" })
        );
    }

    #[test]
    fn list_filters_type_and_content_independently() {
        let params = ListDnsRecordsParams {
            type_filter: Some("MX".into()),
            content: Some(ListDnsRecordsParamsContent {
                exact: Some("mx.example.com".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            serialize_query(&params).unwrap(),
            "type=MX&content.exact=mx.example.com"
        );
    }
}
