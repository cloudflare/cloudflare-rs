use crate::{json_content, Endpoint, JsonResponse, Method};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Workers KV Namespace
///
/// A Namespace is a collection of key-value pairs stored in Workers KV.
/// https://api.cloudflare.com/#workers-kv-namespace-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Namespace {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Create<'a> {
    pub account_id: &'a str,
    pub params: CreateParams,
}
impl Endpoint for Create<'_> {
    type Body = CreateParams;
    type Query = ();
    type Response = JsonResponse<Namespace>;

    const METHOD: Method = Method::Post;

    fn path(&self) -> Cow<str> {
        format!("accounts/{}/storage/kv/namespaces", self.account_id).into()
    }
    fn body(&self) -> &Self::Body {
        &self.params
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateParams {
    pub title: String,
}
json_content!(CreateParams);

#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    pub account_id: &'a str,
    pub params: ListParams,
}
impl Endpoint for List<'_> {
    type Body = ();
    type Query = ListParams;
    type Response = JsonResponse<Vec<Namespace>>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!("accounts/{}/storage/kv/namespaces", self.account_id).into()
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
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Remove<'a> {
    pub account_id: &'a str,
    pub namespace_id: &'a str,
}
impl Endpoint for Remove<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<()>;

    const METHOD: Method = Method::Delete;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/storage/kv/namespaces/{}",
            self.account_id, self.namespace_id
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

#[derive(Debug, Clone, PartialEq)]
pub struct Rename<'a> {
    pub account_id: &'a str,
    pub namespace_id: &'a str,
    pub params: RenameParams,
}
impl Endpoint for Rename<'_> {
    type Body = RenameParams;
    type Query = ();
    type Response = JsonResponse<Namespace>;

    const METHOD: Method = Method::Post;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/storage/kv/namespaces/{}",
            self.account_id, self.namespace_id
        )
        .into()
    }
    fn body(&self) -> &Self::Body {
        &self.params
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}
pub type RenameParams = CreateParams;
