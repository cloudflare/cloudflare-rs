#[cfg(test)]
mod tests {
    use crate::endpoints::d1::{
        CreateDatabaseParams, QueryDatabaseParams, RawQueryParams, 
        D1Database, D1QueryResult, D1RawQueryResult, D1PrimaryLocationHint,
        UpdateDatabaseParams, UpdatePartialDatabaseParams, ExportDatabaseParams,
        ImportDatabaseParams, D1ReadReplicationMode
    };

    #[test]
    fn test_create_database_params() {
        let params = CreateDatabaseParams::new("test-db".to_string());
        assert_eq!(params.name, "test-db");
        assert_eq!(params.primary_location_hint, None);
        
        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{"name":"test-db"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_create_database_params_with_location() {
        let params = CreateDatabaseParams::with_location_hint(
            "test-db".to_string(),
            D1PrimaryLocationHint::Weur
        );
        assert_eq!(params.name, "test-db");
        assert_eq!(params.primary_location_hint, Some(D1PrimaryLocationHint::Weur));
        
        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{"name":"test-db","primary_location_hint":"weur"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_query_database_params() {
        let params = QueryDatabaseParams::new("SELECT * FROM users".to_string());
        assert_eq!(params.sql, "SELECT * FROM users");
        assert!(params.params.is_empty());

        let params_with_bindings = QueryDatabaseParams::with_params(
            "SELECT * FROM users WHERE id = ?".to_string(),
            vec![serde_json::Value::Number(serde_json::Number::from(1))]
        );
        assert_eq!(params_with_bindings.sql, "SELECT * FROM users WHERE id = ?");
        assert_eq!(params_with_bindings.params.len(), 1);
    }

    #[test]
    fn test_raw_query_params() {
        let params = RawQueryParams::new("CREATE TABLE users (id INTEGER PRIMARY KEY)".to_string());
        assert_eq!(params.sql, "CREATE TABLE users (id INTEGER PRIMARY KEY)");
        
        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{"sql":"CREATE TABLE users (id INTEGER PRIMARY KEY)"}"#;
        assert_eq!(json, expected);
    }

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
    fn test_update_database_params() {
        let params = UpdateDatabaseParams::new(D1ReadReplicationMode::Auto);
        
        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{"read_replication":{"mode":"auto"}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_update_partial_database_params() {
        let params = UpdatePartialDatabaseParams::new();
        assert!(params.read_replication.is_none());
        
        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_export_database_params() {
        let params = ExportDatabaseParams::new();
        assert!(params.format.is_none());
        
        let params_with_format = ExportDatabaseParams::with_format("sql".to_string());
        assert_eq!(params_with_format.format, Some("sql".to_string()));
        
        let json = serde_json::to_string(&params_with_format).unwrap();
        let expected = r#"{"format":"sql"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_import_database_params() {
        let params = ImportDatabaseParams::with_sql("CREATE TABLE test (id INTEGER);".to_string());
        assert_eq!(params.sql, Some("CREATE TABLE test (id INTEGER);".to_string()));
        assert!(params.file_name.is_none());
        
        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{"sql":"CREATE TABLE test (id INTEGER);"}"#;
        assert_eq!(json, expected);
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