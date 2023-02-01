use crate::framework::endpoint::{EndpointSpec, Method};

/// Delete a key-value pair from Workers KV
/// Deletes a given key from the given namespace in Workers KV.
/// Returns 404 if the given namespace id is not found for an account.
/// <https://api.cloudflare.com/#workers-kv-namespace-delete-key-value-pair>
#[derive(Debug)]
pub struct DeleteKey<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub key: &'a str,
}

impl<'a> EndpointSpec<()> for DeleteKey<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/values/{}",
            self.account_identifier,
            self.namespace_identifier,
            super::url_encode_key(self.key)
        )
    }
}
