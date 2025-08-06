use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod create_database;
pub mod delete_database;
pub mod export_database;
pub mod get_database;
pub mod import_database;
pub mod list_databases;
pub mod query_database;
pub mod raw_query;
pub mod update_database;
pub mod update_partial_database;

#[cfg(test)]
mod tests;

pub use create_database::{CreateDatabase, CreateDatabaseParams, D1PrimaryLocationHint};
pub use delete_database::DeleteDatabase;
pub use export_database::{ExportDatabase, ExportDatabaseParams, D1ExportResult};
pub use get_database::GetDatabase;
pub use import_database::{ImportDatabase, ImportDatabaseParams, D1ImportResult};
pub use list_databases::ListDatabases;
pub use query_database::{QueryDatabase, QueryDatabaseParams};
pub use raw_query::{RawQuery, RawQueryParams};
pub use update_database::{UpdateDatabase, UpdateDatabaseParams, D1ReadReplicationMode, D1ReadReplicationConfig};
pub use update_partial_database::{UpdatePartialDatabase, UpdatePartialDatabaseParams};

/// D1 Read Replication Details
/// 
/// Configuration details for D1 read replication.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct D1ReadReplicationDetails {
    /// The read replication mode for the database
    pub mode: String,
}

/// D1 Database
/// 
/// Represents a D1 SQLite database instance in Cloudflare's serverless SQL database service.
/// 
/// <https://developers.cloudflare.com/d1/>
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

/// D1 Query Timings
/// 
/// Various durations for the query execution.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct D1QueryTimings {
    /// SQL execution duration in milliseconds (optional)
    pub sql_duration_ms: Option<f64>,
}

/// D1 Query Metadata
/// 
/// Metadata about query execution including performance and change information.
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

impl ApiResult for serde_json::Value {}