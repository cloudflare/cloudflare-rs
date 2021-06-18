use crate::{Endpoint, JsonResponse, Method, OrderDirection};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct List {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub direction: Option<OrderDirection>,
}

impl Endpoint for List {
    type Body = ();
    type Query = Self;
    type Response = JsonResponse<Vec<AccountDetails>>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        "accounts".into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Get<'a> {
    pub account_id: &'a str,
}

impl Endpoint for Get<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<AccountDetails>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!("accounts/{}", self.account_id).into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// Cloudflare Account Details
///
/// An Account is the root object which owns other resources such as zones, load balancers and billing details.
/// https://api.cloudflare.com/#accounts-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AccountDetails {
    /// Account identifier tag.
    pub id: String,
    /// Account name
    pub name: String,
    /// Account Settings
    pub settings: Option<Settings>,
    /// describes when the account was created
    pub created_on: Option<DateTime<Utc>>,
}

/// Cloudflare Accounts Settings
///
/// An object containing the enforce two factor auth property.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Settings {
    /// Indicates whether or not membership in this account requires that Two-Factor Authentication is enabled
    pub enforce_twofactor: bool,
    /// Indicates whether new zones should use the account-level custom nameservers by default
    pub use_account_custom_ns_by_default: bool,
}

/// Cloudflare Accounts
///
/// An Account is the root object which owns other resources such as zones, load balancers and billing details.
/// https://api.cloudflare.com/#accounts-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Account {
    /// Account identifier tag.
    pub id: String,
    /// Account name
    pub name: String,
}
