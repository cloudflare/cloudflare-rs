/*!
D1 database endpoints for Cloudflare's serverless SQL database service.

This module provides comprehensive D1 database management capabilities including:
- Database CRUD operations (create, read, update, delete)  
- SQL query execution (parameterized and raw queries)
- Data import/export operations
- Read replication configuration

All endpoints are fully compliant with the official Cloudflare API specification.
*/

pub mod create_database;
pub mod data_structures;
pub mod delete_database;
pub mod export_database;
pub mod get_database;
pub mod import_database;
pub mod list_databases;
pub mod query_database;
pub mod raw_query;
pub mod update_database;
pub mod update_partial_database;

pub use create_database::{CreateDatabase, CreateDatabaseParams};
pub use data_structures::{
    D1Database, D1ExportResult, D1ImportResult, D1PrimaryLocationHint, D1QueryMeta,
    D1QueryResult, D1QueryTimings, D1RawQueryResult, D1RawQueryResults, D1ReadReplicationConfig,
    D1ReadReplicationDetails, D1ReadReplicationMode,
};
pub use delete_database::DeleteDatabase;
pub use export_database::{ExportDatabase, ExportDatabaseParams};
pub use get_database::GetDatabase;
pub use import_database::{ImportDatabase, ImportDatabaseParams};
pub use list_databases::ListDatabases;
pub use query_database::{QueryDatabase, QueryDatabaseParams};
pub use raw_query::{RawQuery, RawQueryParams};
pub use update_database::{UpdateDatabase, UpdateDatabaseParams};
pub use update_partial_database::{UpdatePartialDatabase, UpdatePartialDatabaseParams};