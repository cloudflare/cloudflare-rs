use crate::framework::endpoint::{serialize_query, EndpointSpec, MultipartBody, MultipartPart};
use crate::framework::endpoint::{Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;
use std::borrow::Cow;

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
    fn body(&self) -> Option<RequestBody> {
        match &self.body {
            WriteKeyBody::Value(value) => Some(RequestBody::Raw(value.clone())),
            WriteKeyBody::Metadata(metadata) => Some(RequestBody::MultiPart(metadata)),
        }
    }
    fn content_type(&self) -> Cow<'static, str> {
        match &self.body {
            WriteKeyBody::Value(_) => Cow::Borrowed("application/octet-stream"),
            WriteKeyBody::Metadata(_) => Cow::Borrowed("multipart/form-data"),
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

impl MultipartBody for WriteKeyBodyMetadata {
    // fn struct_to_multipart(&self) -> Result<Form, reqwest::Error> {
    //     let mut form = Form::new();
    //     form = form.text(
    //         "metadata",
    //         serde_json::to_string(&self.metadata).expect("Failed to serialize metadata"),
    //     );
    //     form = form.part("value", reqwest::multipart::Part::bytes(self.value.clone()));
    //     Ok(form)
    // }

    // Client-agnostic implementation, because of the non-interoperability
    // between reqwest's blocking::multipart::Form/Part and async_impl::multipart::Form/Part.
    // Refactor this when reqwest has some sort of conversion between the two.
    fn parts(&self) -> Vec<(String, MultipartPart)> {
        vec![
            (
                "metadata".to_string(),
                MultipartPart::Text(
                    serde_json::to_string(&self.metadata).expect("Failed to serialize metadata"),
                ),
            ),
            (
                "value".to_string(),
                MultipartPart::Bytes(self.value.clone()),
            ),
        ]
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum WriteKeyBody {
    /// The value to store.
    Value(Vec<u8>),
    /// The value to store with metadata.
    Metadata(WriteKeyBodyMetadata),
}
