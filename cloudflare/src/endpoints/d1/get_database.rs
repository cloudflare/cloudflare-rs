use super::D1Database;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Get details of a specific D1 database
/// 
/// Retrieves detailed information about a D1 database by its UUID.
/// 
/// <https://developers.cloudflare.com/api/resources/d1/methods/get/>
#[derive(Debug)]
pub struct GetDatabase<'a> {
    pub account_identifier: &'a str,
    pub database_identifier: &'a str,
}

impl<'a> GetDatabase<'a> {
    pub fn new(account_identifier: &'a str, database_identifier: &'a str) -> Self {
        Self {
            account_identifier,
            database_identifier,
        }
    }
}

impl EndpointSpec for GetDatabase<'_> {
    type JsonResponse = D1Database;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/d1/database/{}",
            self.account_identifier, self.database_identifier
        )
    }
}