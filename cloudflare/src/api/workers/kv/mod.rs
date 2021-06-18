pub mod namespace;

use crate::{json_content, Endpoint, JsonResponse, Method};
use chrono::{DateTime, Utc};
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
const PATH_ENCODE_SET: &AsciiSet = &QUERY_ENCODE_SET.add(b'?').add(b'`').add(b'{').add(b'}');

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Key {
    pub name: String,
    #[serde(default, with = "crate::serializers::optional_unix_timestamp")]
    pub expiration: Option<DateTime<Utc>>,
}

/// List keys in a namespace
#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    pub account_id: &'a str,
    pub namespace_id: &'a str,
    pub params: ListParams,
}
impl Endpoint for List<'_> {
    type Body = ();
    type Query = ListParams;
    type Response = JsonResponse<Vec<Key>>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/keys",
            self.account_id, self.namespace_id
        )
        .into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &self.params
    }
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
pub struct ListParams {
    pub limit: Option<u16>,
    pub cursor: Option<String>,
    pub prefix: Option<String>,
}

/// Delete a key-value pair from Workers KV
/// Deletes a given key from the given namespace in Workers KV.
/// Returns 404 if the given namespace id is not found for an account.
/// https://api.cloudflare.com/#workers-kv-namespace-delete-key-value-pair
#[derive(Debug, Clone, PartialEq)]
pub struct Delete<'a> {
    pub account_id: &'a str,
    pub namespace_id: &'a str,
    /// The key to delete. This will be URL encoded for you.
    pub key: &'a str,
}

impl Endpoint for Delete<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<()>;

    const METHOD: Method = Method::Delete;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/values/{}",
            self.account_id,
            self.namespace_id,
            percent_encode(self.key.as_bytes(), PATH_ENCODE_SET)
        )
        .into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// Delete Key-Value Pairs in Bulk
/// Deletes multiple key-value pairs from Workers KV at once.
/// A 404 is returned if a delete action is for a namespace ID the account doesn't have.
/// https://api.cloudflare.com/#workers-kv-namespace-delete-multiple-key-value-pairs
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteBulk<'a> {
    pub account_id: &'a str,
    pub namespace_id: &'a str,
    pub keys: Keys,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
pub struct Keys(pub Vec<String>);
json_content!(Keys);

impl Endpoint for DeleteBulk<'_> {
    type Body = Keys;
    type Query = ();
    type Response = JsonResponse<()>;

    const METHOD: Method = Method::Delete;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/bulk",
            self.account_id, self.namespace_id,
        )
        .into()
    }
    fn body(&self) -> &Self::Body {
        &self.keys
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// Write Key-Value Pairs in Bulk
/// Writes multiple key-value pairs to Workers KV at once.
/// A 404 is returned if a write action is for a namespace ID the account doesn't have.
/// https://api.cloudflare.com/#workers-kv-namespace-write-multiple-key-value-pairs
#[derive(Debug, Clone, PartialEq)]
pub struct WriteBulk<'a> {
    pub account_id: &'a str,
    pub namespace_id: &'a str,
    pub values: Values,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
pub struct Values(pub Vec<KeyValuePair>);
json_content!(Values);

impl Endpoint for WriteBulk<'_> {
    type Body = Values;
    type Query = ();
    type Response = JsonResponse<()>;

    const METHOD: Method = Method::Put;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/bulk",
            self.account_id, self.namespace_id,
        )
        .into()
    }
    fn body(&self) -> &Self::Body {
        &self.values
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
    #[serde(default, with = "crate::serializers::optional_unix_timestamp")]
    pub expiration: Option<DateTime<Utc>>,
    #[serde(default, with = "crate::serializers::optional_unix_timestamp")]
    pub expiration_ttl: Option<DateTime<Utc>>,
    pub base64: Option<bool>,
}
