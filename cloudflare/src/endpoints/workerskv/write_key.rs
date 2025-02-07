use crate::framework::endpoint::Method;
use crate::framework::endpoint::{serialize_query, EndpointSpec};
use serde::Serialize;
use std::borrow::Cow;
use crate::framework::response::ApiSuccess;

/// Write a value from Workers KV
/// Returns the value associated with the given key in the given namespace.
/// https://api.cloudflare.com/#workers-kv-namespace-read-key-value-pair
#[derive(Debug)]
pub struct WriteKey<'a> {
    /// Identifier
    pub account_identifier: &'a str,
    /// Namespace identifier tag.
    pub namespace_identifier: &'a str,
    /// A key's name. The name may be at most 512 bytes.
    /// All printable, non-whitespace characters are valid.
    /// Use percent-encoding to define key names as part of a URL.
    pub key: &'a str,
    /// Parameters
    pub params: WriteKeyParams,
    /// Body
    pub body: WriteKeyBody,
}

impl<'a> EndpointSpec for WriteKey<'a> {
    type JsonResponse = ();
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/values/{}",
            self.account_identifier,
            self.namespace_identifier,
            super::url_encode_key(self.key)
        )
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
    #[inline]
    fn body(&self) -> Option<Vec<u8>> {
        match &self.body {
            WriteKeyBody::Value(value) => Some(value.clone()),
            WriteKeyBody::Metadata(metadata) => Some(metadata.value.clone()),
        }
    }
    fn content_type(&self) -> Cow<'static, str> {
        match &self.body {
            // TODO: Check if this works for every case
            WriteKeyBody::Value(_) => Cow::Borrowed("application/octet-stream"),
            // TODO: Check if this is correct, because the documentation says that the content type is multipart/form-data
            WriteKeyBody::Metadata(_) => Cow::Borrowed("application/json"),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct WriteKeyParams {
    /// The time, measured in number of seconds since the UNIX epoch, at which the key should expire.
    pub expiration: Option<i64>,
    /// The number of seconds for which the key should be visible before it expires. At least 60.
    pub expiration_ttl: Option<i64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct WriteKeyBodyMetadata {
    /// The value to store.
    pub value: Vec<u8>,
    /// Arbitrary JSON that is associated with a key.
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Clone, Debug)]
pub enum WriteKeyBody {
    /// The value to store.
    Value(Vec<u8>),
    /// The value to store with metadata.
    Metadata(WriteKeyBodyMetadata),
}
