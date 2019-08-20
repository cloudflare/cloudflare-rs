use super::Key;

use crate::framework::endpoint::{Endpoint, Method};

/// List a Namespace's Keys
/// https://api.cloudflare.com/#workers-kv-namespace-list-a-namespace-s-keys
pub struct ListNamespaceKeys<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub params: &'a ListNamespaceKeysParams<'a>,
}

impl<'a> Endpoint<Vec<Key>, ListNamespaceKeysParams<'a>> for ListNamespaceKeys<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/keys",
            self.account_identifier, self.namespace_identifier
        )
    }
    fn query(&self) -> Option<ListNamespaceKeysParams<'a>> {
        Some(self.params.clone())
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ListNamespaceKeysParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<&'a str>,
}
