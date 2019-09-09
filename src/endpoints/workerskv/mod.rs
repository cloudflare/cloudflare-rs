use crate::framework::response::ApiResult;
use chrono::offset::Utc;
use chrono::DateTime;
use percent_encoding::{percent_encode, PATH_SEGMENT_ENCODE_SET};

pub mod create_namespace;
pub mod delete_bulk;
pub mod delete_key;
pub mod list_namespace_keys;
pub mod list_namespaces;
pub mod remove_namespace;
pub mod rename_namespace;
pub mod write_bulk;

/// Workers KV Namespace
/// A Namespace is a collection of key-value pairs stored in Workers KV.
/// https://api.cloudflare.com/#workers-kv-namespace-properties
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
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
    pub expiration: Option<i64>,
}

impl ApiResult for Key {}
impl ApiResult for Vec<Key> {}

fn url_encode_key(key: &str) -> String {
    percent_encode(key.as_bytes(), PATH_SEGMENT_ENCODE_SET).to_string()
}
