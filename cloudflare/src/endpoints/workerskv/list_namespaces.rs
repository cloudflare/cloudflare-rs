use super::WorkersKvNamespace;

use crate::framework::endpoint::{Endpoint, Method};

/// List Namespaces
/// Returns the namespaces owned by an account
/// https://api.cloudflare.com/#workers-kv-namespace-list-namespaces
#[derive(Debug)]
pub struct ListNamespaces<'a> {
    pub account_identifier: &'a str,
    pub params: ListNamespacesParams,
}

impl<'a> Endpoint<Vec<WorkersKvNamespace>, ListNamespacesParams> for ListNamespaces<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("accounts/{}/storage/kv/namespaces", self.account_identifier)
    }
    fn query(&self) -> Option<ListNamespacesParams> {
        Some(self.params.clone())
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListNamespacesParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
