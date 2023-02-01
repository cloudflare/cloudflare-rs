use super::WorkersKvNamespace;

use crate::framework::endpoint::{EndpointSpec, Method};

use serde::Serialize;

/// Create a Namespace
/// Creates a namespace under the given title.
/// A 400 is returned if the account already owns a namespace with this title.
/// A namespace must be explicitly deleted to be replaced.
/// <https://api.cloudflare.com/#workers-kv-namespace-create-a-namespace>
#[derive(Debug)]
pub struct CreateNamespace<'a> {
    pub account_identifier: &'a str,
    pub params: CreateNamespaceParams,
}

impl<'a> EndpointSpec<WorkersKvNamespace> for CreateNamespace<'a> {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("accounts/{}/storage/kv/namespaces", self.account_identifier)
    }
    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateNamespaceParams {
    pub title: String,
}
