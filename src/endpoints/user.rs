use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::response::ApiResult;

use chrono::{DateTime, Utc};

/// Get User Details
/// Gets information about a user
/// https://api.cloudflare.com/#user-user-details

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct UserDetails {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub telephone: String,
    pub contry: String,
    pub zipcode: String,
    pub created_on: DateTime<Utc>,
    pub modified_on: DateTime<Utc>,
    pub two_factor_authentication_enabled: bool,
    pub suspented: bool,
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