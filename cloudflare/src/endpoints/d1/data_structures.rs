use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// D1 Primary Location Hint
/// 
/// Specify the region to create the D1 primary, if available.
/// If omitted, D1 will be created as close as possible to the current user.
///
/// <https://api.cloudflare.com/#d1-create-database>
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum D1PrimaryLocationHint {
    /// Western North America
    Wnam,
    /// Eastern North America  
    Enam,
    /// Western Europe
    Weur,
    /// Eastern Europe
    Eeur,
    /// Asia-Pacific
    Apac,
    /// Oceania
    Oc,
}

/// D1 Read Replication Mode
/// 
/// Configuration for D1 read replication.
///
/// <https://api.cloudflare.com/#d1-update-database>
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum D1ReadReplicationMode {
    /// Create replicas automatically and place them around the world
    Auto,
    /// Disable database replicas (takes a few hours to delete all replicas)
    Disabled,
}

/// D1 Read Replication Configuration
///
/// <https://api.cloudflare.com/#d1-update-database>
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct D1ReadReplicationConfig {
    /// The read replication mode for the database
    pub mode: D1ReadReplicationMode,
}

/// D1 Read Replication Details
/// 
/// Configuration details for D1 read replication.
///
/// <https://api.cloudflare.com/#d1-get-database>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct D1ReadReplicationDetails {
    /// The read replication mode for the database
    pub mode: String,
}

/// D1 Query Timings
/// 
/// Various durations for the query execution.
///
/// <https://api.cloudflare.com/#d1-query-database>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct D1QueryTimings {
    /// SQL execution duration in milliseconds (optional)
    pub sql_duration_ms: Option<f64>,
}

/// D1 Query Metadata
/// 
/// Metadata about query execution including performance and change information.
///
/// <https://api.cloudflare.com/#d1-query-database>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct D1QueryMeta {
    /// Whether the database has been altered (optional)
    pub changed_db: Option<bool>,
    /// Number of rows changed by the query (optional)
    pub changes: Option<f64>,
    /// Query execution duration in milliseconds (optional)
    pub duration: Option<f64>,
    /// Last inserted row ID (optional)
    pub last_row_id: Option<f64>,
    /// Number of rows read (optional)
    pub rows_read: Option<f64>,
    /// Number of rows written (optional)
    pub rows_written: Option<f64>,
    /// Whether query was served by primary instance (optional)
    pub served_by_primary: Option<bool>,
    /// Region that served the query (optional)
    pub served_by_region: Option<String>,
    /// Database size after the query (optional)
    pub size_after: Option<f64>,
    /// Various query durations (optional)
    pub timings: Option<D1QueryTimings>,
}

/// D1 Database
/// 
/// Represents a D1 SQLite database instance in Cloudflare's serverless SQL database service.
/// 
/// <https://api.cloudflare.com/#d1-list-databases>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct D1Database {
    /// Database UUID identifier
    pub uuid: String,
    /// Human-readable database name
    pub name: String,
    /// Database version (optional)
    pub version: Option<String>,
    /// Number of tables in the database (optional)
    pub num_tables: Option<u32>,
    /// Database file size in bytes (optional)
    pub file_size: Option<u64>,
    /// Region where the database is running (optional)
    pub running_in_region: Option<String>,
    /// Database creation timestamp
    pub created_at: String,
    /// Read replication configuration (optional)
    pub read_replication: Option<D1ReadReplicationDetails>,
}

impl ApiResult for D1Database {}
impl ApiResult for Vec<D1Database> {}

/// D1 Query Result
/// 
/// Response from executing SQL queries against a D1 database.
///
/// <https://api.cloudflare.com/#d1-query-database>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct D1QueryResult {
    /// Query result rows as JSON objects
    pub results: Vec<HashMap<String, serde_json::Value>>,
    /// Query execution metadata
    pub meta: D1QueryMeta,
    /// Whether the query was successful
    pub success: bool,
}

impl ApiResult for D1QueryResult {}
impl ApiResult for Vec<D1QueryResult> {}

/// D1 Raw Query Results
/// 
/// Raw query results with columns and rows as arrays (performance-optimized).
///
/// <https://api.cloudflare.com/#d1-raw-database-query>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct D1RawQueryResults {
    /// Column names
    pub columns: Vec<String>,
    /// Rows as arrays of values
    pub rows: Vec<Vec<serde_json::Value>>,
}

/// D1 Raw Query Result
/// 
/// Response from executing raw SQL queries (performance-optimized format).
///
/// <https://api.cloudflare.com/#d1-raw-database-query>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct D1RawQueryResult {
    /// Raw query results with columns and rows
    pub results: D1RawQueryResults,
    /// Query execution metadata
    pub meta: D1QueryMeta,
    /// Whether the query was successful
    pub success: bool,
}

impl ApiResult for D1RawQueryResult {}

/// D1 Export Result
/// 
/// Result of a D1 database export operation.
///
/// <https://api.cloudflare.com/#d1-export-database>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
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

impl ApiResult for D1ExportResult {}

/// D1 Import Result
/// 
/// Result of a D1 database import operation.
///
/// <https://api.cloudflare.com/#d1-import-database>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct D1ImportResult {
    /// Import operation ID for polling status
    pub id: Option<String>,
    /// Upload URL for the SQL file
    pub upload_url: Option<String>,
    /// Status of the import operation
    pub status: Option<String>,
    /// Import completion time
    pub completed_at: Option<String>,
}

impl ApiResult for D1ImportResult {}

// Implement ApiResult for serde_json::Value for compatibility
impl ApiResult for serde_json::Value {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d1_database_deserialization() {
        let json = r#"
        {
            "uuid": "00000000-0000-0000-0000-000000000000",
            "name": "test-db",
            "version": "1.0",
            "num_tables": 5,
            "file_size": 1024,
            "running_in_region": "weur",
            "created_at": "2024-01-01T00:00:00.000Z",
            "read_replication": {
                "mode": "auto"
            }
        }
        "#;

        let database: D1Database = serde_json::from_str(json).unwrap();
        assert_eq!(database.uuid, "00000000-0000-0000-0000-000000000000");
        assert_eq!(database.name, "test-db");
        assert_eq!(database.version, Some("1.0".to_string()));
        assert_eq!(database.num_tables, Some(5));
        assert_eq!(database.file_size, Some(1024));
        assert_eq!(database.running_in_region, Some("weur".to_string()));
        assert!(database.read_replication.is_some());
        assert_eq!(database.read_replication.unwrap().mode, "auto");
    }

    #[test]
    fn test_d1_query_result_deserialization() {
        let json = r#"
        {
            "results": [
                {"id": 1, "name": "Alice"},
                {"id": 2, "name": "Bob"}
            ],
            "meta": {
                "served_by_region": "WEUR",
                "duration": 15.5,
                "changes": 0,
                "last_row_id": null,
                "changed_db": false,
                "size_after": 2048,
                "rows_read": 2,
                "rows_written": 0,
                "served_by_primary": true
            },
            "success": true
        }
        "#;

        let result: D1QueryResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.results.len(), 2);
        assert_eq!(result.success, true);
        assert_eq!(result.meta.served_by_region, Some("WEUR".to_string()));
        assert_eq!(result.meta.duration, Some(15.5));
        assert_eq!(result.meta.rows_read, Some(2.0));
    }

    #[test]
    fn test_d1_raw_query_result_deserialization() {
        let json = r#"
        {
            "results": {
                "columns": ["id", "name"],
                "rows": [[1, "Alice"], [2, "Bob"]]
            },
            "meta": {
                "served_by_region": "EEUR",
                "duration": 12.3,
                "changes": 0,
                "rows_read": 2,
                "served_by_primary": true
            },
            "success": true
        }
        "#;

        let result: D1RawQueryResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.results.columns, vec!["id", "name"]);
        assert_eq!(result.results.rows.len(), 2);
        assert_eq!(result.success, true);
        assert_eq!(result.meta.served_by_region, Some("EEUR".to_string()));
        assert_eq!(result.meta.duration, Some(12.3));
        assert_eq!(result.meta.served_by_primary, Some(true));
    }
}