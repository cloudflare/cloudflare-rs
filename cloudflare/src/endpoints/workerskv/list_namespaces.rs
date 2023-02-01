use super::WorkersKvNamespace;

use crate::framework::endpoint::{serialize_query, EndpointSpec, Method};

use serde::Serialize;

/// List Namespaces
/// Returns the namespaces owned by an account
/// <https://api.cloudflare.com/#workers-kv-namespace-list-namespaces>
#[derive(Debug)]
pub struct ListNamespaces<'a> {
    pub account_identifier: &'a str,
    pub params: ListNamespacesParams,
}

impl<'a> EndpointSpec<Vec<WorkersKvNamespace>> for ListNamespaces<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("accounts/{}/storage/kv/namespaces", self.account_identifier)
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListNamespacesParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
