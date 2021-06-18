use crate::{json_content, Endpoint, JsonResponse, Method};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Secrets attach to a single script to be readable in only the script
///
/// https://api.cloudflare.com/#worker-secrets-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Secret {
    pub name: String,
    #[serde(rename = "type")]
    pub secret_type: SecretType,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SecretType {
    #[serde(rename = "secret_text")]
    Text,
}

/// Lists all secrets mappings for a given script
///
/// https://api.cloudflare.com/#worker-secrets-list-secrets
#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    pub account_id: &'a str,
    pub script_name: &'a str,
}

impl Endpoint for List<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<Vec<Secret>>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/workers/scripts/{}/secrets",
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

/// Delete Secret
///
/// https://api.cloudflare.com/#worker-delete-secret
#[derive(Debug, Clone, PartialEq)]
pub struct Delete<'a> {
    /// Account id of owner of the script
    pub account_id: &'a str,
    /// The name of the script to remove the secret from
    pub script_name: &'a str,
    /// The variable name of the secret
    pub secret_name: &'a str,
}

impl Endpoint for Delete<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<()>;

    const METHOD: Method = Method::Delete;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/workers/scripts/{}/secrets/{}",
            self.account_id, self.script_name, self.secret_name
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

/// Create Secret
///
/// https://api.cloudflare.com/#worker-create-secret
#[derive(Debug, Clone, PartialEq)]
pub struct Create<'a> {
    /// Account ID of script owner
    pub account_id: &'a str,
    /// The name of the script to attach the secret to
    pub script_name: &'a str,
    /// The contents of the secret
    pub params: CreateParams,
}

impl Endpoint for Create<'_> {
    type Body = CreateParams;
    type Query = ();
    type Response = JsonResponse<()>;

    const METHOD: Method = Method::Put;

    fn path(&self) -> Cow<str> {
        format!(
            "accounts/{}/workers/scripts/{}/secrets",
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
    /// The variable name of the secret that will be bound to the script
    pub name: String,
    #[serde(flatten)]
    pub value: SecretValue,
}
json_content!(CreateParams);

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum SecretValue {
    /// A secret text value
    #[serde(rename = "secret_text")]
    Text {
        /// The string value of the secret
        text: String,
    },
}
