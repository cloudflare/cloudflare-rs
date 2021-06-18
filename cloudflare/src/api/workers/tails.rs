use crate::{json_content, Endpoint, JsonResponse, Method};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A Tail is attached to a single Worker and is impermanent
///
/// https://api.cloudflare.com/#worker-tail-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Tail {
    pub id: String,
    pub url: Option<String>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    pub account_id: &'a str,
    pub script_name: &'a str,
}

impl Endpoint for List<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<Vec<Tail>>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/workers/scripts/{}/tails",
            self.account_id, self.script_name
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

/// Delete Tail
///
/// https://api.cloudflare.com/#worker-delete-tail
#[derive(Debug, Clone, PartialEq)]
pub struct Delete<'a> {
    /// Account id of owner of the script
    pub account_id: &'a str,
    /// The name of the script to remove the Tail session from
    pub script_name: &'a str,
    /// The unique identifier of the Tail session
    pub tail_id: &'a str,
}

impl Endpoint for Delete<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<()>;

    const METHOD: Method = Method::Delete;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/workers/scripts/{}/tails/{}",
            self.account_id, self.script_name, self.tail_id,
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

/// Create Tail
///
/// https://api.cloudflare.com/#worker-create-tail
#[derive(Debug, Clone, PartialEq)]
pub struct Create<'a> {
    /// Account ID of owner of the script
    pub account_id: &'a str,
    /// The name of the script to tail
    pub script_name: &'a str,
    /// V1 of tailing involved creating a separate URL,
    /// which is still possible.
    ///
    /// V2 does not involve a separate URL, so it can
    /// be omitted.
    pub params: CreateParams,
}
impl Endpoint for Create<'_> {
    type Body = CreateParams;
    type Query = ();
    type Response = JsonResponse<Tail>;

    const METHOD: Method = Method::Post;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/workers/scripts/{}/tails",
            self.account_id, self.script_name
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateParams {
    pub url: Option<String>,
}
json_content!(CreateParams);

/// Send Tail Heartbeat
///
/// https://api.cloudflare.com/#worker-tail-heartbeat
#[derive(Debug, Clone, PartialEq)]
pub struct Heartbeat<'a> {
    /// Account id of owner of the script
    pub account_id: &'a str,
    /// The name of the script to remove the Tail session from
    pub script_name: &'a str,
    /// The unique identifier of the Tail session
    pub tail_id: &'a str,
}

impl Endpoint for Heartbeat<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<Tail>;

    const METHOD: Method = Method::Post;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/workers/scripts/{}/tails/{}",
            self.account_id, self.script_name, self.tail_id,
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
