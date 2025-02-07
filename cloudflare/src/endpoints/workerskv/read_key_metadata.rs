use crate::framework::endpoint::EndpointSpec;
use crate::framework::endpoint::Method;
use crate::framework::response::{ApiResult, ApiSuccess};

/// Read a value from Workers KV
/// Returns the value associated with the given key in the given namespace.
/// https://api.cloudflare.com/#workers-kv-namespace-read-key-value-pair
#[derive(Debug)]
pub struct ReadKeyMetadata<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub key: &'a str,
}

impl ApiResult for Option<serde_json::Value> {}

impl<'a> EndpointSpec for ReadKeyMetadata<'a> {
    // TODO: Option<HashMap<String, serde_json::Value>> or Option<serde_json::Value>? (test it)
    type JsonResponse = Option<serde_json::Value>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/metadata/{}",
            self.account_identifier,
            self.namespace_identifier,
            super::url_encode_key(self.key)
        )
    }
}
