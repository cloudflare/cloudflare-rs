use chrono::{DateTime, Utc};
use cloudflare_derive_macros::{ApiResult, VecApiResult};
use serde::{Deserialize, Serialize};

pub mod list_accounts;
pub use list_accounts::ListAccounts;

/// Cloudflare Accounts
/// An Account is the root object which owns other resources such as zones, load balancers and billing details.
/// <https://api.cloudflare.com/#accounts-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, ApiResult, VecApiResult)]
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
/// <https://api.cloudflare.com/#accounts-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AccountDetails {
    /// Account identifier tag.
    pub id: String,
    /// Account name
    pub name: String,
}
