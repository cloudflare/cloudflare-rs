use crate::framework::response::ApiResult;
use chrono::DateTime;
use chrono::{TimeZone, Utc};
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Deserializer};

pub mod create_namespace;
pub mod delete_bulk;
pub mod delete_key;
pub mod list_namespace_keys;
pub mod list_namespaces;
pub mod remove_namespace;
pub mod rename_namespace;
pub mod write_bulk;

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
/// A Namespace is a collection of key-value pairs stored in Workers KV.
/// https://api.cloudflare.com/#workers-kv-namespace-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersKvNamespace {
    /// Namespace identifier tag.
    pub id: String,
    /// A human-readable string name for a Namespace.
    pub title: String,
}

impl ApiResult for WorkersKvNamespace {}

impl ApiResult for Vec<WorkersKvNamespace> {}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Key {
    pub name: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_timestamp")]
    pub expiration: Option<DateTime<Utc>>,
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
