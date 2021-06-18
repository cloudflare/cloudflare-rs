use crate::{Endpoint, JsonResponse, Method};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrganizationStatus {
    Member,
    Invited,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Organization {
    pub id: String,
    pub name: String,
    /// Whether or not the user is a member of the organization or has an inivitation pending
    pub status: OrganizationStatus,
    /// Access permissions for this User
    pub permissions: Vec<String>,
    /// List of role names for the User at the Organization
    pub roles: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserDetails {
    pub organizations: Vec<Organization>,
    #[serde(default)]
    pub betas: Vec<String>,
    pub telephone: Option<String>,
    pub zipcode: Option<String>,
    pub last_name: Option<String>,
    pub modified_on: DateTime<Utc>,
    pub username: String,
    pub created_on: DateTime<Utc>,
    pub country: Option<String>,
    pub two_factor_authentication_enabled: bool,
    pub first_name: Option<String>,
    pub id: String,
    pub suspended: bool,
    pub email: String,
}

/// Get User Details
///
/// Gets information about a user
/// https://api.cloudflare.com/#user-user-details
#[derive(Debug, Clone, PartialEq)]
pub struct Get;
impl Endpoint for Get {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<UserDetails>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        "user".into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TokenStatus {
    Initialized,
    Active,
    Disabled,
    Expired,
    Revoked,
    Purged,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserTokenStatus {
    pub id: String,
    pub status: TokenStatus,
    pub not_before: Option<DateTime<Utc>>,
    pub expires_on: Option<DateTime<Utc>>,
}

/// Validate User Token
///
/// Returns whether a given token is valid or not.
/// https://blog.cloudflare.com/api-tokens-general-availability/
#[derive(Debug, Clone, PartialEq)]
pub struct VerifyToken;
impl Endpoint for VerifyToken {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<UserTokenStatus>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        "user/tokens/verify".into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}
