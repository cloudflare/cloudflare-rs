use crate::endpoint::{Endpoint, Method};
use crate::response::APIResult;
use chrono::offset::Utc;
use chrono::DateTime;

/// Workers KV Namespace
/// A Namespace is a collection of key-value pairs stored in Workers KV.
/// https://api.cloudflare.com/#workers-kv-namespace-properties
#[derive(Deserialize, Debug)]
pub struct WorkersKVNamespace {
    /// Namespace identifier tag.
    pub id: String,
    /// A human-readable string name for a Namespace.
    pub title: String,
}

impl APIResult for WorkersKVNamespace {}
impl APIResult for Vec<WorkersKVNamespace> {}

/// List Namespaces
/// Returns the namespaces owned by an account
/// https://api.cloudflare.com/#workers-kv-namespace-list-namespaces
pub struct ListNamespaces<'a> {
    pub account_identifier: &'a str,
}
impl<'a> Endpoint<Vec<WorkersKVNamespace>, ListNamespacesParams> for ListNamespaces<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("accounts/{}/storage/kv/namespaces", self.account_identifier)
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ListNamespacesParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// Create a Namespace
/// Creates a namespace under the given title.
/// A 400 is returned if the account already owns a namespace with this title.
/// A namespace must be explicitly deleted to be replaced.
/// https://api.cloudflare.com/#workers-kv-namespace-create-a-namespace
pub struct CreateNamespace<'a> {
    pub account_identifier: &'a str,
}
impl<'a> Endpoint<WorkersKVNamespace, CreateNamespaceParams> for CreateNamespace<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("accounts/{}/storage/kv/namespaces", self.account_identifier)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateNamespaceParams {
    pub title: String,
}

/// Remove a Namespace
/// Deletes the namespace corresponding to the given ID.
/// https://api.cloudflare.com/#workers-kv-namespace-remove-a-namespace
pub struct RemoveNamespace<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
}
impl<'a> Endpoint for RemoveNamespace<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}",
            self.account_identifier, self.namespace_identifier
        )
    }
}

/// Rename a Namespace
/// Modifies a namespace's title.
/// https://api.cloudflare.com/#workers-kv-namespace-rename-a-namespace
pub struct RenameNamespace<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
}
impl<'a> Endpoint<(), RenameNamespaceParams> for RenameNamespace<'a> {
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}",
            self.account_identifier, self.namespace_identifier
        )
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct RenameNamespaceParams {
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct Key {
    pub name: String,
    pub expiration: Option<DateTime<Utc>>,
}
impl APIResult for Key {}
impl APIResult for Vec<Key> {}

/// List a Namespace's Keys
/// https://api.cloudflare.com/#workers-kv-namespace-list-a-namespace-s-keys
pub struct ListNamespaceKeys<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
}
impl<'a> Endpoint<Vec<Key>, ListNamespaceKeysParams> for ListNamespaceKeys<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/keys",
            self.account_identifier, self.namespace_identifier
        )
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ListNamespaceKeysParams {
    pub limit: Option<u16>,
    pub cursor: Option<String>,
}
