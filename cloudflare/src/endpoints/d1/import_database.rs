use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Import SQL into a D1 database
/// 
/// Generates a temporary URL for uploading an SQL file to, then instructing
/// the D1 to import it and polling it for status updates. Imports block
/// the D1 for their duration.
/// 
/// <https://api.cloudflare.com/#d1-import-database>
#[derive(Debug)]
pub struct ImportDatabase<'a> {
    pub account_identifier: &'a str,
    pub database_identifier: &'a str,
    pub params: ImportDatabaseParams,
}

impl<'a> ImportDatabase<'a> {
    pub fn new(
        account_identifier: &'a str,
        database_identifier: &'a str,
        params: ImportDatabaseParams,
    ) -> Self {
        Self {
            account_identifier,
            database_identifier,
            params,
        }
    }
}

impl EndpointSpec for ImportDatabase<'_> {
    type JsonResponse = D1ImportResult;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/d1/database/{}/import",
            self.account_identifier, self.database_identifier
        )
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

use super::data_structures::D1ImportResult;

/// Parameters for importing to a D1 database
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct ImportDatabaseParams {
    /// SQL content to import (optional - if not provided, use upload_url)
    pub sql: Option<String>,
    /// File name for the import operation (optional)
    pub file_name: Option<String>,
}

impl ImportDatabaseParams {
    pub fn new() -> Self {
        Self { 
            sql: None,
            file_name: None,
        }
    }
    
    pub fn with_sql(sql: String) -> Self {
        Self {
            sql: Some(sql),
            file_name: None,
        }
    }
    
    pub fn with_file_name(file_name: String) -> Self {
        Self {
            sql: None,
            file_name: Some(file_name),
        }
    }
}

impl Default for ImportDatabaseParams {
    fn default() -> Self {
        Self::new()
    }
}