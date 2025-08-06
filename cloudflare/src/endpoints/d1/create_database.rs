use super::data_structures::{D1Database, D1PrimaryLocationHint};

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Create a new D1 database
/// 
/// Creates a new D1 database with the specified name.
/// Database names must be unique within the account.
/// 
/// <https://api.cloudflare.com/#d1-create-database>
#[derive(Debug)]
pub struct CreateDatabase<'a> {
    pub account_identifier: &'a str,
    pub params: CreateDatabaseParams,
}

impl<'a> CreateDatabase<'a> {
    pub fn new(account_identifier: &'a str, params: CreateDatabaseParams) -> Self {
        Self {
            account_identifier,
            params,
        }
    }
}

impl EndpointSpec for CreateDatabase<'_> {
    type JsonResponse = D1Database;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("accounts/{}/d1/database", self.account_identifier)
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Parameters for creating a D1 database
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct CreateDatabaseParams {
    /// The name of the database to create
    pub name: String,
    /// Specify the region to create the D1 primary (optional)
    pub primary_location_hint: Option<D1PrimaryLocationHint>,
}

impl CreateDatabaseParams {
    pub fn new(name: String) -> Self {
        Self { 
            name,
            primary_location_hint: None,
        }
    }
    
    pub fn with_location_hint(name: String, location_hint: D1PrimaryLocationHint) -> Self {
        Self {
            name,
            primary_location_hint: Some(location_hint),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}