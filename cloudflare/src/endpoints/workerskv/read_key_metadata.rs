use crate::framework::endpoint::EndpointSpec;
use crate::framework::endpoint::Method;
use crate::framework::response::{ApiResult, ApiSuccess};

/// Returns the metadata associated with the given key in the given namespace.
///
/// Use URL-encoding to use special characters (for example, `:`, `!`, `%`) in the key name.
///
/// <https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/metadata/methods/get/>
#[derive(Debug)]
pub struct ReadKeyMetadata<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub key: &'a str,
}

impl ApiResult for Option<serde_json::Value> {}

impl<'a> EndpointSpec for ReadKeyMetadata<'a> {
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
