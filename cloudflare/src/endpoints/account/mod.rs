use crate::framework::response::ApiResult;
use chrono::{DateTime, Utc};
use serde::Deserialize;

pub mod list_accounts;
pub use list_accounts::ListAccounts;

/// Cloudflare Accounts
/// An Account is the root object which owns other resources such as zones, load balancers and billing details.
/// https://api.cloudflare.com/#accounts-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Account {
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
/// An object containing the enforce two factor auth property.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    /// Indicates whether or not membership in this account requires that Two-Factor Authentication is enabled
    enforce_twofactor: bool,
}

/// Cloudflare Accounts Details
/// An Account is the root object which owns other resources such as zones, load balancers and billing details.
/// https://api.cloudflare.com/#accounts-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AccountDetails {
    /// Account identifier tag.
    pub id: String,
    /// Account name
    pub name: String,
}

impl ApiResult for Account {}
impl ApiResult for Vec<Account> {}
