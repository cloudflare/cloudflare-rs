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
    status: String,
    permissions: Vec<String>,
    roles: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct UserDetails {
    pub organizations: Vec<Organization>,
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

pub struct GetUserDetails {}

impl<'a> Endpoint<UserDetails, (), ()> for GetUserDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        "user".to_string()
    }
}
