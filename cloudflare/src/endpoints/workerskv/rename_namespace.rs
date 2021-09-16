use crate::framework::endpoint::{Endpoint, Method};

/// Rename a Namespace
/// Modifies a namespace's title.
/// https://api.cloudflare.com/#workers-kv-namespace-rename-a-namespace
#[derive(Debug)]
pub struct RenameNamespace<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub params: RenameNamespaceParams,
}

impl<'a> Endpoint<(), (), RenameNamespaceParams> for RenameNamespace<'a> {
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}",
            self.account_identifier, self.namespace_identifier
        )
    }
    fn body(&self) -> Option<RenameNamespaceParams> {
        Some(self.params.clone())
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct RenameNamespaceParams {
    pub title: String,
}
