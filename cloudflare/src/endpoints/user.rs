use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::response::ApiResult;

use chrono::{DateTime, Utc};

/// Get User Details
/// Gets information about a user
/// https://api.cloudflare.com/#user-user-details

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Organization {
    id: String,
    name: String,
    status: String, // Whether or not the user is a member of the organization or has an inivitation pending
    permissions: Vec<String>, // Access permissions for this User
    roles: Vec<String>, // List of role names for the User at the Organization
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
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
impl ApiResult for UserDetails {}

#[test]
fn handles_empty_betas_field() {
    // note: omitted `betas` field from json data
    const JSON_RESPONSE: &str = r#"
    {
        "id": "1234567890abcdef",
        "email": "user@example.com",
        "username": "user",
        "first_name": null,
        "last_name": null,
        "telephone": null,
        "country": null,
        "zipcode": null,
        "two_factor_authentication_enabled": false,
        "two_factor_authentication_locked": false,
        "created_on": "2015-02-24T13:03:05.255956Z",
        "modified_on": "2018-06-10T23:50:04.029596Z",
        "organizations": [],
        "has_pro_zones": false,
        "has_business_zones": false,
        "has_enterprise_zones": false,
        "suspended": false
    }"#;

    let user_details: UserDetails = serde_json::from_str(JSON_RESPONSE).unwrap();
    assert!(user_details.betas.is_empty());
}

#[derive(Debug)]
pub struct GetUserDetails {}

impl Endpoint<UserDetails, (), ()> for GetUserDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        "user".to_string()
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

#[derive(Debug)]
pub struct GetUserTokenStatus {}

impl Endpoint<UserTokenStatus, (), ()> for GetUserTokenStatus {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        "user/tokens/verify".to_string()
    }
}
