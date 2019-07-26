use crate::response::APIResult;
use chrono::offset::Utc;
use chrono::DateTime;

mod create_namespace;
mod list_namespace_keys;
mod list_namespaces;
mod remove_namespace;
mod rename_namespace;

pub use create_namespace::CreateNamespace;
pub use list_namespace_keys::ListNamespaceKeys;
pub use list_namespaces::ListNamespaces;
pub use remove_namespace::RemoveNamespace;
pub use rename_namespace::RenameNamespace;

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
