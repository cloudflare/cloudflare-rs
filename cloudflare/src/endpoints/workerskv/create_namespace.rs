use super::WorkersKvNamespace;

use crate::framework::endpoint::{Endpoint, Method};

/// Create a Namespace
/// Creates a namespace under the given title.
/// A 400 is returned if the account already owns a namespace with this title.
/// A namespace must be explicitly deleted to be replaced.
/// https://api.cloudflare.com/#workers-kv-namespace-create-a-namespace
#[derive(Debug)]
pub struct CreateNamespace<'a> {
    pub account_identifier: &'a str,
    pub params: CreateNamespaceParams,
}

impl<'a> Endpoint<WorkersKvNamespace, (), CreateNamespaceParams> for CreateNamespace<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("accounts/{}/storage/kv/namespaces", self.account_identifier)
    }
    fn body(&self) -> Option<CreateNamespaceParams> {
        Some(self.params.clone())
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateNamespaceParams {
    pub title: String,
}
