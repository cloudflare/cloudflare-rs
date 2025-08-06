use super::data_structures::{D1Database, D1ReadReplicationConfig};

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Partially update a D1 database
/// 
/// Partially updates configuration for an existing D1 database.
/// Only provided fields will be updated.
/// 
/// <https://api.cloudflare.com/#d1-update-partial-database>
#[derive(Debug)]
pub struct UpdatePartialDatabase<'a> {
    pub account_identifier: &'a str,
    pub database_identifier: &'a str,
    pub params: UpdatePartialDatabaseParams,
}

impl<'a> UpdatePartialDatabase<'a> {
    pub fn new(
        account_identifier: &'a str,
        database_identifier: &'a str,
        params: UpdatePartialDatabaseParams,
    ) -> Self {
        Self {
            account_identifier,
            database_identifier,
            params,
        }
    }
}

impl EndpointSpec for UpdatePartialDatabase<'_> {
    type JsonResponse = D1Database;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/d1/database/{}",
            self.account_identifier, self.database_identifier
        )
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Parameters for partially updating a D1 database
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdatePartialDatabaseParams {
    /// Configuration for D1 read replication (optional)
    pub read_replication: Option<D1ReadReplicationConfig>,
}

impl UpdatePartialDatabaseParams {
    pub fn new() -> Self {
        Self {
            read_replication: None,
        }
    }
    
    pub fn with_read_replication(read_replication: D1ReadReplicationConfig) -> Self {
        Self {
            read_replication: Some(read_replication),
        }
    }
}

impl Default for UpdatePartialDatabaseParams {
    fn default() -> Self {
        Self::new()
    }
}