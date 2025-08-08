use super::data_structures::D1RawQueryResult;

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Execute raw SQL against a D1 database
/// 
/// Executes raw SQL statements against the specified D1 database.
/// This endpoint is useful for administrative operations and bulk operations
/// but should be used with caution as it doesn't support parameterization.
/// 
/// <https://api.cloudflare.com/#d1-raw-database-query>
#[derive(Debug)]
pub struct RawQuery<'a> {
    pub account_identifier: &'a str,
    pub database_identifier: &'a str,
    pub params: RawQueryParams,
}

impl<'a> RawQuery<'a> {
    pub fn new(
        account_identifier: &'a str,
        database_identifier: &'a str,
        params: RawQueryParams,
    ) -> Self {
        Self {
            account_identifier,
            database_identifier,
            params,
        }
    }
}

impl EndpointSpec for RawQuery<'_> {
    type JsonResponse = D1RawQueryResult;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/d1/database/{}/raw",
            self.account_identifier, self.database_identifier
        )
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Parameters for executing raw SQL
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct RawQueryParams {
    /// The raw SQL to execute
    pub sql: String,
}

impl RawQueryParams {
    pub fn new(sql: String) -> Self {
        Self { sql }
    }
}