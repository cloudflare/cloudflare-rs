use crate::framework::endpoint::{Endpoint, Method};

/// Delete Key-Value Pairs in Bulk
/// Deletes multiple key-value pairs from Workers KV at once.
/// A 404 is returned if a delete action is for a namespace ID the account doesn't have.
/// https://api.cloudflare.com/#workers-kv-namespace-delete-multiple-key-value-pairs
#[derive(Debug)]
pub struct DeleteBulk<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub bulk_keys: Vec<String>,
}

impl<'a> Endpoint<(), (), Vec<String>> for DeleteBulk<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/bulk",
            self.account_identifier, self.namespace_identifier
        )
    }
    fn body(&self) -> Option<Vec<String>> {
        Some(self.bulk_keys.clone())
    }
    // default content-type is already application/json
}
