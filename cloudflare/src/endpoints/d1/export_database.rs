use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Export a D1 database as SQL
/// 
/// Returns a URL where the SQL contents of your D1 can be downloaded.
/// Note: this process may take some time for larger DBs, during which
/// your D1 will be unavailable to serve queries. To avoid blocking
/// your DB unnecessarily, an in-progress export must be continually
/// polled or will automatically cancel.
/// 
/// <https://api.cloudflare.com/#d1-export-database>
#[derive(Debug)]
pub struct ExportDatabase<'a> {
    pub account_identifier: &'a str,
    pub database_identifier: &'a str,
    pub params: ExportDatabaseParams,
}

impl<'a> ExportDatabase<'a> {
    pub fn new(
        account_identifier: &'a str,
        database_identifier: &'a str,
        params: ExportDatabaseParams,
    ) -> Self {
        Self {
            account_identifier,
            database_identifier,
            params,
        }
    }
}

impl EndpointSpec for ExportDatabase<'_> {
    type JsonResponse = D1ExportResult;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/d1/database/{}/export",
            self.account_identifier, self.database_identifier
        )
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

use super::data_structures::D1ExportResult;

/// Parameters for exporting a D1 database
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct ExportDatabaseParams {
    /// Output format for the export (optional)
    pub format: Option<String>,
}

impl ExportDatabaseParams {
    pub fn new() -> Self {
        Self { format: None }
    }
    
    pub fn with_format(format: String) -> Self {
        Self {
            format: Some(format),
        }
    }
}

impl Default for ExportDatabaseParams {
    fn default() -> Self {
        Self::new()
    }
}