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
/// <https://developers.cloudflare.com/api/resources/d1/methods/export/>
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

/// D1 Export Result
/// 
/// Result of a D1 database export operation.
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct D1ExportResult {
    /// Export operation ID for polling status
    pub id: Option<String>,
    /// URL to download the exported SQL file (when ready)
    pub url: Option<String>,
    /// Status of the export operation
    pub status: Option<String>,
    /// Export expiry time
    pub expires_at: Option<String>,
}

impl crate::framework::response::ApiResult for D1ExportResult {}

/// Parameters for exporting a D1 database
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct ExportDatabaseParams {
    /// Output format for the export (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
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