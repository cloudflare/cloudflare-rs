use crate::framework::endpoint::EndpointSpec;
use crate::framework::endpoint::Method;
use crate::framework::response::ApiResult;

/// Returns the value associated with the given key in the given namespace.
///
/// Use URL-encoding to use special characters (for example, `:`, `!`, `%`) in the key name.
/// If the KV-pair is set to expire at some point, the expiration time as measured in seconds since
/// the UNIX epoch will be returned in the expiration response header.
///
/// <https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/values/methods/get/>
#[derive(Debug)]
pub struct ReadKey<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub key: &'a str,
}

impl ApiResult for Vec<u8> {}

impl EndpointSpec for ReadKey<'_> {
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
