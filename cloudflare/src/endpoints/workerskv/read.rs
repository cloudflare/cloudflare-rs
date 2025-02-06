use crate::framework::endpoint::EndpointSpec;
use crate::framework::response::ApiResult;
use crate::framework::endpoint::Method;

/// Read a value from Workers KV
/// Returns the value associated with the given key in the given namespace.
/// https://api.cloudflare.com/#workers-kv-namespace-read-key-value-pair
#[derive(Debug)]
pub struct Read<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub key: &'a str,
}

impl ApiResult for Vec<u8> {}

impl<'a> EndpointSpec for Read<'a> {
    const IS_RAW_BODY: bool = true;

    type JsonResponse = ();
    type ResponseType = Vec<u8>;

    fn method(&self) -> Method {
        Method::GET
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
