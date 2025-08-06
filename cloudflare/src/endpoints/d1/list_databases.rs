use super::D1Database;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// List all D1 databases in an account
/// 
/// Returns a list of all D1 databases owned by the account.
/// 
/// <https://developers.cloudflare.com/api/resources/d1/methods/list/>
#[derive(Debug)]
pub struct ListDatabases<'a> {
    pub account_identifier: &'a str,
}

impl<'a> ListDatabases<'a> {
    pub fn new(account_identifier: &'a str) -> Self {
        Self {
            account_identifier,
        }
    }
}

impl EndpointSpec for ListDatabases<'_> {
    type JsonResponse = Vec<D1Database>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}/d1/database", self.account_identifier)
    }
}