use super::Key;

use crate::framework::endpoint::{serialize_query, EndpointSpec, Method};

use serde::Serialize;

/// List a Namespace's Keys
/// <https://api.cloudflare.com/#workers-kv-namespace-list-a-namespace-s-keys>
#[derive(Debug)]
pub struct ListNamespaceKeys<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub params: ListNamespaceKeysParams,
}

impl<'a> EndpointSpec<Vec<Key>> for ListNamespaceKeys<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/keys",
            self.account_identifier, self.namespace_identifier
        )
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListNamespaceKeysParams {
    pub limit: Option<u16>,
    pub cursor: Option<String>,
    pub prefix: Option<String>,
}
