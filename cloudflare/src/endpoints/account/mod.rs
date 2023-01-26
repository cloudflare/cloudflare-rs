use crate::framework::response::ApiResult;
use serde::Deserialize;
use serde_with::serde_as;
use time::OffsetDateTime;

pub mod list_accounts;
pub use list_accounts::ListAccounts;

/// Cloudflare Accounts
/// An Account is the root object which owns other resources such as zones, load balancers and billing details.
/// https://api.cloudflare.com/#accounts-properties
#[serde_as]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Account {
    /// Account identifier tag.
    pub id: String,
    /// Account name
    pub name: String,
    /// Account Settings
    pub settings: Option<Settings>,
    /// describes when the account was created
    #[serde_as(as = "Option<time::format_description::well_known::Rfc3339>")]
    pub created_on: Option<OffsetDateTime>,
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
