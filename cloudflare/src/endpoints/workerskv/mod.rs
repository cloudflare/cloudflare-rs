use crate::framework::response::ApiResult;
use chrono::DateTime;
use chrono::{TimeZone, Utc};
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
    urlencoding::encode(key).to_string()
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersKvBulkResult {
    /// Number of keys successfully updated.
    pub successful_key_count: Option<i8>,

    /// Name of the keys that failed to be fully updated. They should be retried.
    // TODO: Ambiguity with the official docs; it does not seem to be optional. It's an empty array if no keys failed.
    pub unsuccessful_keys: Option<Vec<String>>,
}

impl ApiResult for WorkersKvBulkResult {}
