use super::data_structures::{D1Database, D1ReadReplicationMode, D1ReadReplicationConfig};

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Update a D1 database
/// 
/// Updates configuration for an existing D1 database.
/// 
/// <https://api.cloudflare.com/#d1-update-database>
#[derive(Debug)]
pub struct UpdateDatabase<'a> {
    pub account_identifier: &'a str,
    pub database_identifier: &'a str,
    pub params: UpdateDatabaseParams,
}

impl<'a> UpdateDatabase<'a> {
    pub fn new(
        account_identifier: &'a str,
        database_identifier: &'a str,
        params: UpdateDatabaseParams,
    ) -> Self {
        Self {
            account_identifier,
            database_identifier,
            params,
        }
    }
}

impl EndpointSpec for UpdateDatabase<'_> {
    type JsonResponse = D1Database;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
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

/// Parameters for updating a D1 database
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateDatabaseParams {
    /// Configuration for D1 read replication
    pub read_replication: D1ReadReplicationConfig,
}

impl UpdateDatabaseParams {
    pub fn new(mode: D1ReadReplicationMode) -> Self {
        Self {
            read_replication: D1ReadReplicationConfig { mode },
        }
    }
}