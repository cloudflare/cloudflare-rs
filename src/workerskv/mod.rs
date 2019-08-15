use crate::response::ApiResult;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::ser::{Serialize, Serializer};

pub mod create_namespace;
pub mod list_namespace_keys;
pub mod list_namespaces;
pub mod remove_namespace;
pub mod rename_namespace;
pub mod get_value;

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

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Key {
    pub name: String,
    pub expiration: Option<DateTime<Utc>>,
}

impl ApiResult for Key {}
impl ApiResult for Vec<Key> {}
