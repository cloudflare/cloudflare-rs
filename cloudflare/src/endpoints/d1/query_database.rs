use super::{D1QueryResult};

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Execute a parameterized SQL query against a D1 database
/// 
/// Executes a SQL query with optional parameters against the specified D1 database.
/// This is the recommended way to execute queries as it supports parameterization
/// which helps prevent SQL injection attacks.
/// 
/// <https://developers.cloudflare.com/api/resources/d1/methods/query/>
#[derive(Debug)]
pub struct QueryDatabase<'a> {
    pub account_identifier: &'a str,
    pub database_identifier: &'a str,
    pub params: QueryDatabaseParams,
}

impl<'a> QueryDatabase<'a> {
    pub fn new(
        account_identifier: &'a str,
        database_identifier: &'a str,
        params: QueryDatabaseParams,
    ) -> Self {
        Self {
            account_identifier,
            database_identifier,
            params,
        }
    }
}

impl EndpointSpec for QueryDatabase<'_> {
    type JsonResponse = Vec<D1QueryResult>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/d1/database/{}/query",
            self.account_identifier, self.database_identifier
        )
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Parameters for executing a parameterized SQL query
#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct QueryDatabaseParams {
    /// The SQL statement to execute
    pub sql: String,
    /// Parameters to bind to the SQL statement
    pub params: Vec<serde_json::Value>,
}

impl QueryDatabaseParams {
    pub fn new(sql: String) -> Self {
        Self {
            sql,
            params: vec![],
        }
    }

    pub fn with_params(sql: String, params: Vec<serde_json::Value>) -> Self {
        Self { sql, params }
    }
}