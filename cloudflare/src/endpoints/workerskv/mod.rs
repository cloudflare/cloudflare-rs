use crate::framework::response::ApiResult;
use chrono::DateTime;
use chrono::{TimeZone, Utc};
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Deserializer, Serialize};

pub mod create_namespace;
pub mod delete_bulk;
pub mod delete_key;
pub mod get_namespace;
pub mod list_namespace_keys;
pub mod list_namespaces;
pub mod read_key;
pub mod read_key_metadata;
pub mod remove_namespace;
pub mod rename_namespace;
pub mod write_bulk;
pub mod write_key;
// Upgrading to percent_encode 2.x unfortunately removed this prebaked const.
// We need to re-assemble it by combining "control" ASCII characters with other characters
// which are invalid or reserved in URIs. Non-ASCII characters are always encoded.

// https://docs.rs/percent-encoding/1.0.0/src/percent_encoding/lib.rs.html#104
const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &CONTROLS
    // "QUERY_ENCODE_SET" additions:
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    // "DEFAULT_ENCODE_SET" additions:
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    // "PATH_SEGMENT_ENCODE_SET" additions
    .add(b'%')
    .add(b'/')
    // The following were NOT in PATH_SEGMENT but are URI reserved characters not covered above.
    // ':' and '@' are explicitly permitted in paths, so we don't add them.
    .add(b'[')
    .add(b']');

/// Workers KV Namespace
/// 
/// A Namespace is a collection of key-value pairs stored in Workers KV.
/// 
/// <https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/models/namespace/#(schema)>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersKvNamespace {
    /// Namespace identifier tag.
    pub id: String,
    /// A human-readable string name for a Namespace.
    pub title: String,
    /// True if keys written on the URL will be URL-decoded before storing.
    /// For example, if set to "true", a key written on the URL as "%3F" will be stored as "?".
    pub supports_url_encoding: Option<bool>,
}

impl ApiResult for WorkersKvNamespace {}

impl ApiResult for Vec<WorkersKvNamespace> {}

/// A name for a value. A value stored under a given key may be retrieved via the same key.
#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Key {
    /// A key's name. The name may be at most 512 bytes.
    /// All printable, non-whitespace characters are valid.
    /// Use percent-encoding to define key names as part of a URL.
    pub name: String,

    /// The time, measured in number of seconds since the UNIX epoch, at which the key will expire.
    /// This property is omitted for keys that will not expire.
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_timestamp")]
    pub expiration: Option<DateTime<Utc>>,

    // TODO: Option<HashMap<String, serde_json::Value>> or Option<serde_json::Value>? (test it)
    /// Arbitrary JSON that is associated with a key.
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

pub fn deserialize_option_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<i64> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        return Ok(Utc.timestamp_opt(s, 0).single());
    }

    Ok(None)
}

impl ApiResult for Key {}

impl ApiResult for Vec<Key> {}

fn url_encode_key(key: &str) -> String {
    percent_encode(key.as_bytes(), PATH_SEGMENT_ENCODE_SET).to_string()
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersKvBulkResult {
    /// Number of keys successfully updated.
    successful_key_count: Option<i8>,

    /// Name of the keys that failed to be fully updated. They should be retried.
    unsuccessful_keys: Option<Vec<String>>,
}

impl ApiResult for WorkersKvBulkResult {}
