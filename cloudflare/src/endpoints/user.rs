use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::response::ApiResult;

use chrono::{DateTime, Utc};

// The currently logged in/authenticated User
// https://api.cloudflare.com/#user-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct UserDetails {
    // A list of the organizations the user is a member of (or invited to) and the permissions granted to them
    pub organizations: Vec<Organization>,
    // A list of betas the user is currently participating in. If a beta is zone-specific, the beta will apply to all zones
    pub betas: Vec<String>,
    // User's telephone number
    pub telephone: Option<String>,
    // The zipcode or postal code where the user lives
    pub zipcode: Option<String>,
    // User's last name
    pub last_name: Option<String>,
    // Last time the user was modified
    pub modified_on: DateTime<Utc>,
    // A username used to access other cloudflare services, like support
    pub username: String,
    // When the user signed up
    pub created_on: DateTime<Utc>,
    // The country in which the user lives - ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
    // Whether two-factor authentication is enabled for the user account. This does not apply to API authentication
    pub two_factor_authentication_enabled: bool,
    // User's first name
    pub first_name: Option<String>,
    // User identifier tag
    pub id: String,
    // Indicates whether the user is prevented from performing certain actions within their account
    pub suspended: bool,
    // User's email address
    pub email: String,
}
impl ApiResult for UserDetails {}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Organization {
    // Organization identifier tag
    id: String,
    // Organization Name
    name: String,
    // Whether or not the user is a member of the organization or has an invitation pending
    status: String,
    // Access permissions for this User
    permissions: Vec<String>, 
    // List of role names for the User at the Organization
    roles: Vec<String>, 
}

/// Get User Details
/// Gets information about a user
/// https://api.cloudflare.com/#user-user-details
pub struct GetUserDetails {}

impl<'a> Endpoint<UserDetails, (), ()> for GetUserDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        "user".to_string()
    }
}

/// Edit User Details
/// Edit part of your user details
/// https://api.cloudflare.com/#user-edit-user
pub struct EditUserDetails {
    pub params: EditUserDetailsParams,
}

#[derive(Serialize, Clone, Debug)]
pub struct EditUserDetailsParams {
    // User's first name
    pub first_name: Option<String>,
    // User's last name
    pub last_name: Option<String>,
    // User's telephone number
    pub telephone: Option<String>,
    // The country in which the user lives - ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
    // The zipcode or postal code where the user lives
    pub zipcode: Option<String>,
}

impl<'a> Endpoint<UserDetails, (), EditUserDetailsParams> for EditUserDetails {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        "user".to_string()
    }
    fn body(&self) -> Option<EditUserDetailsParams> {
        Some(self.params.clone())
    }
}

/// Validate User Token
/// Returns whether a given token is valid or not.
/// https://blog.cloudflare.com/api-tokens-general-availability/
///
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct UserTokenStatus {
    pub id: String,
    pub status: String,
}
impl ApiResult for UserTokenStatus {}

pub struct GetUserTokenStatus {}

impl<'a> Endpoint<UserTokenStatus, (), ()> for GetUserTokenStatus {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        "user/tokens/verify".to_string()
    }
}
