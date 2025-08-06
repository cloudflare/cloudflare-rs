use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Delete a D1 database
/// 
/// Permanently deletes a D1 database and all its data.
/// This operation cannot be undone.
/// 
/// <https://api.cloudflare.com/#d1-delete-database>
#[derive(Debug)]
pub struct DeleteDatabase<'a> {
    pub account_identifier: &'a str,
    pub database_identifier: &'a str,
}

impl<'a> DeleteDatabase<'a> {
    pub fn new(account_identifier: &'a str, database_identifier: &'a str) -> Self {
        Self {
            account_identifier,
            database_identifier,
        }
    }
}

impl EndpointSpec for DeleteDatabase<'_> {
    type JsonResponse = serde_json::Value;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/d1/database/{}",
            self.account_identifier, self.database_identifier
        )
    }
}