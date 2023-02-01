use crate::framework::endpoint::{EndpointSpec, Method};

use serde::Serialize;

/// Rename a Namespace
/// Modifies a namespace's title.
/// <https://api.cloudflare.com/#workers-kv-namespace-rename-a-namespace>
#[derive(Debug)]
pub struct RenameNamespace<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub params: RenameNamespaceParams,
}

impl<'a> EndpointSpec<()> for RenameNamespace<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}",
            self.account_identifier, self.namespace_identifier
        )
    }
    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct RenameNamespaceParams {
    pub title: String,
}
