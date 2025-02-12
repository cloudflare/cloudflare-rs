/*!
This module controls how requests are sent to Cloudflare's API, and how responses are parsed from it.
 */
pub mod auth;
pub mod client;
pub mod endpoint;
pub mod response;

use serde::Serialize;

#[derive(thiserror::Error, Debug)]
/// Errors encountered while trying to connect to the Cloudflare API
pub enum Error {
    /// An error via the `reqwest` crate
    #[error("Reqwest returned an error when connecting to the Cloudflare API: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Serialize, Clone, Debug)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// Used as a parameter to API calls that search for a resource (e.g. DNS records).
/// Tells the API whether to return results that match all search requirements or at least one (any).
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SearchMatch {
    /// Match all search requirements
    All,
    /// Match at least one search requirement
    Any,
}

/// Which environment (host path) to use for API calls
#[derive(Debug)]
pub enum Environment {
    /// The production endpoint: `https://api.cloudflare.com/client/v4`
    Production,
    /// A custom endpoint (for example, a `mockito` server)
    Custom(String),
}

impl<'a> From<&'a Environment> for url::Url {
    fn from(environment: &Environment) -> Self {
        match environment {
            Environment::Production => {
                url::Url::parse("https://api.cloudflare.com/client/v4/").unwrap()
            }
            Environment::Custom(url) => url::Url::parse(url.as_str()).unwrap(),
        }
    }
}
