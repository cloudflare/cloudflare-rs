use super::WorkersKVNamespace;

use crate::endpoint::{Endpoint, Method};

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
