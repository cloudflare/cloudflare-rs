use crate::framework::endpoint::{Endpoint, Method};

/// Remove a Namespace
/// Deletes the namespace corresponding to the given ID.
/// https://api.cloudflare.com/#workers-kv-namespace-remove-a-namespace
#[derive(Debug)]
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
