use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Remove a KV pair from the namespace.
///
/// Use URL-encoding to use special characters (for example, `:`, `!`, `%`) in the key name.
///
/// <https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/values/methods/delete/>
#[derive(Debug)]
pub struct DeleteKey<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub key: &'a str,
}

impl<'a> EndpointSpec for DeleteKey<'a> {
    type JsonResponse = ();
    type ResponseType = ApiSuccess<Self::JsonResponse>;

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
