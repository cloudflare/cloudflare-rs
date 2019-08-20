use crate::framework::endpoint::{Endpoint, Method};

/// Write Key-Value Pairs in Bulk
/// Writes multiple key-value pairs to Workers KV at once.
/// A 404 is returned if a write action is for a namespace ID the account doesn't have.
/// https://api.cloudflare.com/#workers-kv-namespace-write-multiple-key-value-pairs
pub struct WriteBulk<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub bulk_key_value_pairs: Vec<KeyValuePair>,
}

impl<'a> Endpoint<(), (), Vec<KeyValuePair>> for WriteBulk<'a> {
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/bulk",
            self.account_identifier, self.namespace_identifier
        )
    }
    fn body(&self) -> Option<Vec<KeyValuePair>> {
        Some(self.bulk_key_value_pairs.clone())
    }
    // default content-type is already application/json
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_ttl: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64: Option<bool>,
}
