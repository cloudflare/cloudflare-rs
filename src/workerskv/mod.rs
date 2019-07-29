use crate::response::APIResult;
use chrono::offset::Utc;
use chrono::DateTime;

pub mod create_namespace;
pub mod list_namespace_keys;
pub mod list_namespaces;
pub mod remove_namespace;
pub mod rename_namespace;

/// Workers KV Namespace
/// A Namespace is a collection of key-value pairs stored in Workers KV.
/// https://api.cloudflare.com/#workers-kv-namespace-properties
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkersKVNamespace {
    /// Namespace identifier tag.
    pub id: String,
    /// A human-readable string name for a Namespace.
    pub title: String,
}

impl APIResult for WorkersKVNamespace {}
impl APIResult for Vec<WorkersKVNamespace> {}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Key {
    pub name: String,
    pub expiration: Option<DateTime<Utc>>,
}

impl APIResult for Key {}
impl APIResult for Vec<Key> {}
